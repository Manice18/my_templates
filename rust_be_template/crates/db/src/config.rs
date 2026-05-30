use anyhow::Result;
use sqlx::{Pool, Postgres};

#[derive(Clone, Debug)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout_seconds: u64,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self> {
        let url = std::env::var("DATABASE_URL")
            .map_err(|_| anyhow::anyhow!("DATABASE_URL must be set"))?;

        Ok(Self::new(url))
    }

    pub fn new(url: impl Into<String>) -> Self {
        let max_connections = std::env::var("DB_MAX_CONNECTIONS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10);

        let min_connections = std::env::var("DB_MIN_CONNECTIONS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0)
            .min(max_connections);

        let connection_timeout_seconds = std::env::var("DB_CONNECTION_TIMEOUT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30);

        Self {
            url: url.into(),
            max_connections,
            min_connections,
            connection_timeout_seconds,
        }
    }
}

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<Postgres>,
}

impl Database {
    pub async fn connect(config: DatabaseConfig) -> Result<Self> {
        tracing::info!(
            max_connections = config.max_connections,
            min_connections = config.min_connections,
            acquire_timeout_seconds = config.connection_timeout_seconds,
            "Connecting to database"
        );

        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(std::time::Duration::from_secs(
                config.connection_timeout_seconds,
            ))
            .connect(&config.url)
            .await
            .map_err(|e| {
                anyhow::anyhow!(
                    "Failed to connect to database: {e}. \
                     Ensure PostgreSQL is running and DATABASE_URL is correct \
                     (local dev: make services-start)"
                )
            })?;

        tracing::info!("Database pool ready");

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }

    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(|e| anyhow::anyhow!("Database health check failed: {e}"))?;
        Ok(())
    }

    pub async fn run_migrations(&self, migrations_path: &str) -> Result<()> {
        tracing::info!("Running database migrations...");

        sqlx::migrate::Migrator::new(std::path::Path::new(migrations_path))
            .await?
            .run(self.pool())
            .await
            .map_err(|e| anyhow::anyhow!("Migration failed: {e}"))?;

        tracing::info!("Database migrations completed");
        Ok(())
    }
}
