use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, Mutex},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use payment_core::{
    PlatformError,
    types::{TenantId, UserId},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{middleware::errors::ApiError, state::AppState};

const TOKEN_LIFETIME_SECONDS: u64 = 900;
const LOGIN_WINDOW: Duration = Duration::from_secs(60);
const LOGIN_ATTEMPT_LIMIT: u8 = 10;
const MAX_TRACKED_LOGIN_IDENTITIES: usize = 10_000;

#[derive(Clone)]
pub struct AuthService {
    secret: String,
    issuer: String,
    dummy_password_hash: String,
    login_attempts: Arc<Mutex<HashMap<String, LoginAttemptWindow>>>,
}

#[derive(Debug)]
struct LoginAttemptWindow {
    started_at: Instant,
    attempts: u8,
}

impl fmt::Debug for AuthService {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthService")
            .field("secret", &"[REDACTED]")
            .field("issuer", &self.issuer)
            .finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TokenClaims {
    sub: String,
    tenant_id: String,
    role: String,
    iss: String,
    iat: u64,
    exp: u64,
}

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: UserId,
    pub tenant_id: TenantId,
    pub role: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct IssuedToken {
    pub access_token: String,
    pub token_type: &'static str,
    pub expires_in: u64,
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub role: String,
}

impl AuthService {
    pub fn new(secret: String, issuer: String) -> Result<Self, PlatformError> {
        if secret.as_bytes().len() < 32 {
            return Err(PlatformError::Validation(
                "JWT_SECRET must contain at least 32 bytes".into(),
            ));
        }
        if issuer.trim().is_empty() {
            return Err(PlatformError::Validation(
                "JWT_ISSUER must not be empty".into(),
            ));
        }
        let salt = SaltString::generate(&mut OsRng);
        let dummy_password_hash = Argon2::default()
            .hash_password(b"dummy-account-password", &salt)
            .map_err(|error| PlatformError::Internal(error.to_string()))?
            .to_string();
        Ok(Self {
            secret,
            issuer,
            dummy_password_hash,
            login_attempts: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub fn dummy_password_hash(&self) -> &str {
        &self.dummy_password_hash
    }

    pub fn record_login_attempt(&self, identity: &str) -> Result<(), PlatformError> {
        let now = Instant::now();
        let mut attempts = self
            .login_attempts
            .lock()
            .map_err(|_| PlatformError::Internal("Login rate limiter is unavailable".into()))?;

        if attempts
            .get(identity)
            .is_some_and(|window| now.duration_since(window.started_at) >= LOGIN_WINDOW)
        {
            attempts.remove(identity);
        }

        if let Some(window) = attempts.get_mut(identity) {
            if window.attempts >= LOGIN_ATTEMPT_LIMIT {
                return Err(PlatformError::RateLimited);
            }
            window.attempts += 1;
            return Ok(());
        }

        if attempts.len() >= MAX_TRACKED_LOGIN_IDENTITIES {
            attempts.retain(|_, window| now.duration_since(window.started_at) < LOGIN_WINDOW);
        }
        if attempts.len() >= MAX_TRACKED_LOGIN_IDENTITIES {
            return Err(PlatformError::RateLimited);
        }

        attempts.insert(
            identity.to_owned(),
            LoginAttemptWindow {
                started_at: now,
                attempts: 1,
            },
        );
        Ok(())
    }

    pub fn clear_login_attempts(&self, identity: &str) -> Result<(), PlatformError> {
        self.login_attempts
            .lock()
            .map_err(|_| PlatformError::Internal("Login rate limiter is unavailable".into()))?
            .remove(identity);
        Ok(())
    }

    pub fn issue(
        &self,
        user_id: UserId,
        tenant_id: TenantId,
        role: String,
    ) -> Result<IssuedToken, PlatformError> {
        if !matches!(
            role.as_str(),
            "admin" | "operator" | "risk_analyst" | "read_only"
        ) {
            return Err(PlatformError::Forbidden(
                "User role is not permitted to access the API".into(),
            ));
        }
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| PlatformError::Internal(error.to_string()))?
            .as_secs();
        let claims = TokenClaims {
            sub: user_id.0.to_string(),
            tenant_id: tenant_id.0.to_string(),
            role: role.clone(),
            iss: self.issuer.clone(),
            iat: now,
            exp: now + TOKEN_LIFETIME_SECONDS,
        };
        let access_token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|error| PlatformError::Internal(error.to_string()))?;

        Ok(IssuedToken {
            access_token,
            token_type: "Bearer",
            expires_in: TOKEN_LIFETIME_SECONDS,
            user_id: user_id.0,
            tenant_id: tenant_id.0,
            role,
        })
    }

    pub fn verify(&self, token: &str) -> Result<AuthenticatedUser, PlatformError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&[self.issuer.as_str()]);
        validation.leeway = 0;
        let claims = decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )
        .map_err(|_| PlatformError::Unauthorized("Invalid or expired access token".into()))?
        .claims;

        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| PlatformError::Unauthorized("Invalid access token".into()))?;
        let tenant_id = Uuid::parse_str(&claims.tenant_id)
            .map_err(|_| PlatformError::Unauthorized("Invalid access token".into()))?;
        if !matches!(
            claims.role.as_str(),
            "admin" | "operator" | "risk_analyst" | "read_only"
        ) {
            return Err(PlatformError::Unauthorized(
                "Invalid access token role".into(),
            ));
        }

        Ok(AuthenticatedUser {
            user_id: UserId(user_id),
            tenant_id: TenantId(tenant_id),
            role: claims.role,
        })
    }
}

impl AuthenticatedUser {
    pub fn can_evaluate_payments(&self) -> bool {
        matches!(self.role.as_str(), "admin" | "operator" | "risk_analyst")
    }
}

impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| {
                ApiError(PlatformError::Unauthorized(
                    "Bearer access token is required".into(),
                ))
            })?;
        let token = header.strip_prefix("Bearer ").ok_or_else(|| {
            ApiError(PlatformError::Unauthorized(
                "Bearer access token is required".into(),
            ))
        })?;

        state.auth.verify(token).map_err(ApiError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn service() -> AuthService {
        AuthService::new(
            "a-development-secret-at-least-32-bytes-long".into(),
            "test".into(),
        )
        .expect("valid test auth configuration")
    }

    #[test]
    fn issued_token_preserves_verified_identity_and_role() {
        let service = service();
        let user_id = UserId(Uuid::new_v4());
        let tenant_id = TenantId(Uuid::new_v4());
        let token = service
            .issue(user_id, tenant_id, "operator".into())
            .expect("token should issue");

        let verified = service
            .verify(&token.access_token)
            .expect("token should verify");
        assert_eq!(verified.user_id.0, user_id.0);
        assert_eq!(verified.tenant_id.0, tenant_id.0);
        assert_eq!(verified.role, "operator");
    }

    #[test]
    fn rejects_tampered_tokens() {
        let service = service();
        let token = service
            .issue(
                UserId(Uuid::new_v4()),
                TenantId(Uuid::new_v4()),
                "operator".into(),
            )
            .expect("token should issue");

        let tampered = format!("{}x", token.access_token);
        assert!(service.verify(&tampered).is_err());
    }

    #[test]
    fn refuses_weak_signing_secrets() {
        assert!(AuthService::new("short".into(), "test".into()).is_err());
    }

    #[test]
    fn rejects_tokens_from_another_issuer() {
        let service = service();
        let claims = TokenClaims {
            sub: Uuid::new_v4().to_string(),
            tenant_id: Uuid::new_v4().to_string(),
            role: "operator".into(),
            iss: "another-service".into(),
            iat: 1,
            exp: u64::MAX,
        };
        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(b"a-development-secret-at-least-32-bytes-long"),
        )
        .expect("token should encode");

        assert!(service.verify(&token).is_err());
    }

    #[test]
    fn rejects_expired_tokens() {
        let service = service();
        let claims = TokenClaims {
            sub: Uuid::new_v4().to_string(),
            tenant_id: Uuid::new_v4().to_string(),
            role: "operator".into(),
            iss: "test".into(),
            iat: 1,
            exp: 1,
        };
        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(b"a-development-secret-at-least-32-bytes-long"),
        )
        .expect("token should encode");

        assert!(service.verify(&token).is_err());
    }

    #[test]
    fn read_only_users_cannot_evaluate_payments() {
        let service = service();
        let token = service
            .issue(
                UserId(Uuid::new_v4()),
                TenantId(Uuid::new_v4()),
                "read_only".into(),
            )
            .expect("token should issue");

        let verified = service
            .verify(&token.access_token)
            .expect("token should verify");
        assert!(!verified.can_evaluate_payments());
    }

    #[test]
    fn limits_repeated_login_attempts_per_identity() {
        let service = service();
        for _ in 0..LOGIN_ATTEMPT_LIMIT {
            service
                .record_login_attempt("operator@example.test")
                .expect("attempt should be within the limit");
        }
        assert!(matches!(
            service.record_login_attempt("operator@example.test"),
            Err(PlatformError::RateLimited)
        ));
        service
            .record_login_attempt("another@example.test")
            .expect("limit should be independent per identity");
    }

    #[test]
    fn clears_failed_attempt_counter_after_authentication() {
        let service = service();
        for _ in 0..LOGIN_ATTEMPT_LIMIT {
            service
                .record_login_attempt("operator@example.test")
                .expect("attempt should be within the limit");
        }
        service
            .clear_login_attempts("operator@example.test")
            .expect("attempt state should clear");
        service
            .record_login_attempt("operator@example.test")
            .expect("authenticated user should be able to retry");
    }
}
