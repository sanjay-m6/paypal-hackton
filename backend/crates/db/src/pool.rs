use payment_core::PlatformError;

/// A thin wrapper around the PostgreSQL connection pool.
#[derive(Debug, Clone)]
pub struct DbPool {
    pool: sqlx::PgPool,
}

impl DbPool {
    /// Connect to the database using the provided URL.
    pub async fn connect(database_url: &str) -> Result<Self, PlatformError> {
        let pool = sqlx::PgPool::connect(database_url)
            .await
            .map_err(|e| PlatformError::DatabaseError(e.to_string()))?;

        tracing::info!("Database connection pool established");
        Ok(Self { pool })
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
