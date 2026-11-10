use std::{env, error::Error, sync::Arc};

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use payment_agents::registry::AgentRegistry;
use payment_api::{build_router, middleware::auth::AuthService, state::AppState};
use payment_audit::logger::AuditLogger;
use payment_core::AgentMessageEnvelope;
use payment_db::{
    pool::{DbPool, PoolConfig},
    repository::UserRepository,
};
use payment_messaging::{
    JetStreamBus, MessageKeyring, MessageValidationConfig, MessageValidator, MessagingConfig,
};
use payment_policy::engine::PolicyEngine;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    tracing::info!("Starting Multi-Agent Payment Security Platform API (Axum)");

    let database_url = env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL must be configured before starting the API")?;
    let jwt_secret = env::var("JWT_SECRET")
        .map_err(|_| "JWT_SECRET must be configured before starting the API")?;
    let jwt_issuer = env::var("JWT_ISSUER").unwrap_or_else(|_| "payment-security-api".into());
    let auth = AuthService::new(jwt_secret, jwt_issuer)?;

    let max_connections = match env::var("DATABASE_MAX_CONNECTIONS") {
        Ok(value) => value.parse::<u32>()?,
        Err(env::VarError::NotPresent) => 10,
        Err(error) => return Err(error.into()),
    };
    if max_connections == 0 {
        return Err("DATABASE_MAX_CONNECTIONS must be greater than zero".into());
    }
    let db = DbPool::connect_with_config(
        &database_url,
        PoolConfig {
            max_connections,
            min_connections: max_connections.min(2),
            ..PoolConfig::default()
        },
    )
    .await?;
    db.run_migrations().await?;
    bootstrap_local_operator(&db).await?;
    let messaging_enabled = match env::var("MESSAGING_ENABLED") {
        Ok(value) if value == "true" => true,
        Ok(value) if value == "false" => false,
        Ok(_) => return Err("MESSAGING_ENABLED must be either true or false".into()),
        Err(env::VarError::NotPresent) => false,
        Err(error) => return Err(error.into()),
    };
    let (messaging, messaging_validator) = if messaging_enabled {
        let key_json = env::var("MESSAGE_SIGNING_KEYS_JSON")
            .map_err(|_| "MESSAGE_SIGNING_KEYS_JSON is required when messaging is enabled")?;
        let keyring = MessageKeyring::from_json(&key_json)?;
        let messaging_config = MessagingConfig::from_env()?;
        let validator = Arc::new(MessageValidator::new(
            keyring,
            MessageValidationConfig {
                max_payload_bytes: messaging_config.max_payload_bytes,
                max_ttl: messaging_config.message_ttl,
                ..MessageValidationConfig::default()
            },
        ));
        let bus = Arc::new(JetStreamBus::connect(messaging_config).await?);
        (Some(bus), Some(validator))
    } else {
        (None, None)
    };

    let state = AppState {
        db,
        auth,
        policy_engine: Arc::new(PolicyEngine::new()),
        agent_registry: Arc::new(AgentRegistry::new()),
        audit_logger: Arc::new(AuditLogger::new()),
        messaging,
        messaging_validator,
    };
    if let (Some(bus), Some(validator)) =
        (state.messaging.as_ref(), state.messaging_validator.as_ref())
    {
        tokio::spawn(run_outbox_publisher(
            state.db.clone(),
            Arc::clone(bus),
            Arc::clone(validator),
        ));
    }
    let app = build_router(state);

    // Start server
    let host = std::env::var("API_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("API_PORT").unwrap_or_else(|_| "3001".to_string());
    let addr = format!("{host}:{port}");

    tracing::info!("Listening on {addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn run_outbox_publisher(
    db: DbPool,
    bus: Arc<JetStreamBus>,
    validator: Arc<MessageValidator>,
) {
    let repository = payment_db::messages::MessageRepository::new(db.inner().clone());
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
    let mut next_retention_cleanup = tokio::time::Instant::now();
    loop {
        interval.tick().await;
        let tenants = match repository.pending_tenants(100).await {
            Ok(tenants) => tenants,
            Err(error) => {
                tracing::error!(error = %error, "Unable to load message outbox tenants");
                continue;
            }
        };
        if tokio::time::Instant::now() >= next_retention_cleanup {
            if let Err(error) = repository.purge_expired_records().await {
                tracing::error!(error = %error, "Unable to purge expired message records");
            }
            next_retention_cleanup =
                tokio::time::Instant::now() + std::time::Duration::from_secs(24 * 60 * 60);
        }
        for tenant_id in tenants {
            if let Err(error) = repository.expire_pending_outbox(tenant_id).await {
                tracing::error!(
                    tenant_id = %tenant_id.0,
                    error = %error,
                    "Unable to dead-letter expired outbox messages"
                );
            }
            let messages = match repository.pending_outbox(tenant_id, 100).await {
                Ok(messages) => messages,
                Err(error) => {
                    tracing::error!(tenant_id = %tenant_id.0, error = %error, "Unable to load tenant message outbox");
                    continue;
                }
            };
            for entry in messages {
                let envelope = match serde_json::from_value::<AgentMessageEnvelope>(entry.envelope)
                {
                    Ok(envelope) => envelope,
                    Err(_) => {
                        record_outbox_failure(
                            &repository,
                            entry.tenant_id,
                            entry.message_id,
                            "invalid_envelope",
                        )
                        .await;
                        continue;
                    }
                };
                if envelope.tenant_id.0 != entry.tenant_id
                    || envelope.message_id != entry.message_id
                    || envelope.task_id != entry.task_id
                {
                    record_outbox_failure(
                        &repository,
                        entry.tenant_id,
                        entry.message_id,
                        "context_mismatch",
                    )
                    .await;
                    continue;
                }
                let route_matches = matches!(
                    payment_messaging::bus::subject_for_envelope(&envelope),
                    Ok(subject) if subject == entry.subject
                );
                if !route_matches {
                    record_outbox_failure(
                        &repository,
                        entry.tenant_id,
                        entry.message_id,
                        "route_mismatch",
                    )
                    .await;
                    continue;
                }
                if validator.validate(&envelope, chrono::Utc::now()).is_err() {
                    record_outbox_failure(
                        &repository,
                        entry.tenant_id,
                        entry.message_id,
                        "validation_failed",
                    )
                    .await;
                    continue;
                }
                if bus.publish(&envelope, &validator).await.is_err() {
                    record_outbox_failure(
                        &repository,
                        entry.tenant_id,
                        entry.message_id,
                        "publish_failed",
                    )
                    .await;
                    continue;
                }
                match repository
                    .mark_outbox_published(
                        payment_core::types::TenantId(entry.tenant_id),
                        entry.message_id,
                    )
                    .await
                {
                    Ok(true) => {}
                    Ok(false) => tracing::warn!(
                        tenant_id = %entry.tenant_id,
                        message_id = %entry.message_id,
                        "Published message was not marked in the outbox"
                    ),
                    Err(error) => tracing::error!(
                        tenant_id = %entry.tenant_id,
                        message_id = %entry.message_id,
                        error = %error,
                        "Unable to mark published message in the outbox"
                    ),
                }
            }
        }
    }
}

async fn record_outbox_failure(
    repository: &payment_db::messages::MessageRepository,
    tenant_id: uuid::Uuid,
    message_id: uuid::Uuid,
    safe_code: &str,
) {
    match repository
        .record_outbox_failure(
            payment_core::types::TenantId(tenant_id),
            message_id,
            safe_code,
        )
        .await
    {
        Ok(true) => {}
        Ok(false) => tracing::warn!(
            tenant_id = %tenant_id,
            message_id = %message_id,
            "Outbox failure was not recorded because the message is unavailable or complete"
        ),
        Err(error) => tracing::error!(
            tenant_id = %tenant_id,
            message_id = %message_id,
            error = %error,
            "Unable to record message outbox failure"
        ),
    }
}

async fn bootstrap_local_operator(db: &DbPool) -> Result<(), Box<dyn Error>> {
    let email = env::var("BOOTSTRAP_ADMIN_EMAIL").unwrap_or_default();
    let password = env::var("BOOTSTRAP_ADMIN_PASSWORD").unwrap_or_default();

    if email.is_empty() && password.is_empty() {
        return Ok(());
    }
    if email.is_empty() || password.len() < 12 {
        return Err(
            "Set both BOOTSTRAP_ADMIN_EMAIL and a BOOTSTRAP_ADMIN_PASSWORD of at least 12 characters"
                .into(),
        );
    }

    let password_hash = tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
    })
    .await?
    .map_err(|error| std::io::Error::other(error.to_string()))?;
    let tenant_id = env::var("BOOTSTRAP_TENANT_ID")
        .unwrap_or_else(|_| "22222222-2222-4222-8222-222222222222".into())
        .parse::<Uuid>()?;

    UserRepository::new(db.inner().clone())
        .bootstrap_operator(&email, &password_hash, tenant_id)
        .await?;
    tracing::info!(tenant_id = %tenant_id, "Local bootstrap operator is available");
    Ok(())
}
