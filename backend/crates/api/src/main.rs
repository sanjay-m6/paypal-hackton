use std::{env, error::Error, sync::Arc};

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use payment_agents::registry::AgentRegistry;
use payment_api::{build_router, middleware::auth::AuthService, state::AppState};
use payment_audit::logger::AuditLogger;
use payment_db::{
    pool::{DbPool, PoolConfig},
    repository::UserRepository,
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

    let state = AppState {
        db,
        auth,
        policy_engine: Arc::new(PolicyEngine::new()),
        agent_registry: Arc::new(AgentRegistry::new()),
        audit_logger: Arc::new(AuditLogger::new()),
    };
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
