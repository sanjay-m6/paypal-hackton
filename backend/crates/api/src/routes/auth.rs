use argon2::{Argon2, PasswordVerifier};
use axum::{Json, extract::State, extract::rejection::JsonRejection};
use password_hash::PasswordHash;
use payment_core::{
    PlatformError,
    types::{TenantId, UserId},
};
use payment_db::repository::UserRepository;
use serde::{Deserialize, Serialize};

use crate::{middleware::errors::ApiError, state::AppState};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    #[serde(flatten)]
    token: crate::middleware::auth::IssuedToken,
}

pub async fn login(
    State(state): State<AppState>,
    payload: Result<Json<LoginRequest>, JsonRejection>,
) -> Result<Json<LoginResponse>, ApiError> {
    let Json(payload) = payload.map_err(ApiError::from)?;
    let email = payload.email.trim().to_ascii_lowercase();
    if email.is_empty() || email.len() > 254 || payload.password.is_empty() {
        return Err(ApiError(PlatformError::Validation(
            "A valid email and password are required".into(),
        )));
    }
    if payload.password.len() > 1024 {
        return Err(ApiError(PlatformError::Validation(
            "Password is too long".into(),
        )));
    }
    state.auth.record_login_attempt(&email)?;

    let user = UserRepository::new(state.db.inner().clone())
        .get_active_auth_user(&email)
        .await?;

    let password = payload.password;
    let password_hash = user
        .as_ref()
        .map(|record| record.password_hash.clone())
        .unwrap_or_else(|| state.auth.dummy_password_hash().to_owned());
    let password_matches = tokio::task::spawn_blocking(move || {
        let parsed = PasswordHash::new(&password_hash).map_err(|error| {
            tracing::error!("Stored password hash is invalid: {error}");
            PlatformError::Internal("Stored password hash is invalid".into())
        })?;
        Ok::<bool, PlatformError>(
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed)
                .is_ok(),
        )
    })
    .await
    .map_err(|error| {
        tracing::error!("Password verification task failed: {error}");
        ApiError(PlatformError::Internal(
            "Password verification failed".into(),
        ))
    })??;

    let Some(user) = user.filter(|_| password_matches) else {
        return Err(ApiError(PlatformError::Unauthorized(
            "Invalid email or password".into(),
        )));
    };

    let token = state
        .auth
        .issue(UserId(user.id), TenantId(user.tenant_id), user.role)?;
    state.auth.clear_login_attempts(&email)?;
    Ok(Json(LoginResponse { token }))
}
