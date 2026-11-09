use payment_core::PlatformError;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

/// Configuration options for the database connection pool.
#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout_secs: u64,
    pub idle_timeout_secs: u64,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 10,
            min_connections: 2,
            acquire_timeout_secs: 5,
            idle_timeout_secs: 600,
        }
    }
}

/// A robust wrapper around the PostgreSQL connection pool.
#[derive(Debug, Clone)]
pub struct DbPool {
    pool: sqlx::PgPool,
}

impl DbPool {
    /// Connect to the database with production pool tuning.
    pub async fn connect(database_url: &str) -> Result<Self, PlatformError> {
        Self::connect_with_config(database_url, PoolConfig::default()).await
    }

    /// Connect with explicit pool configuration.
    pub async fn connect_with_config(
        database_url: &str,
        config: PoolConfig,
    ) -> Result<Self, PlatformError> {
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(Duration::from_secs(config.acquire_timeout_secs))
            .idle_timeout(Duration::from_secs(config.idle_timeout_secs))
            .connect(database_url)
            .await
            .map_err(|e| PlatformError::DatabaseError(e.to_string()))?;

        tracing::info!(
            max_connections = config.max_connections,
            "Database connection pool established"
        );
        Ok(Self { pool })
    }

    /// Health check ping executing a lightweight query.
    pub async fn check_health(&self) -> Result<(), PlatformError> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(|e| PlatformError::DatabaseError(format!("Healthcheck failed: {e}")))?;
        Ok(())
    }

    /// Run embedded SQLx migrations.
    pub async fn run_migrations(&self) -> Result<(), PlatformError> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(|e| PlatformError::DatabaseError(e.to_string()))?;

        tracing::info!("Database migrations applied successfully");
        Ok(())
    }

    /// Returns a reference to the underlying connection pool.
    pub fn inner(&self) -> &sqlx::PgPool {
        &self.pool
    }
}
