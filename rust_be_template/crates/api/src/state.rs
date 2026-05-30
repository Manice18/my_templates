use anyhow::Result;
use db::{ApiKeyQueries, BusinessQueries, Database, DatabaseConfig};

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub business_queries: BusinessQueries,
    pub api_key_queries: ApiKeyQueries,
}

impl AppState {
    pub async fn new(database_url: &str) -> Result<Self> {
        let db_config = DatabaseConfig::new(database_url);
        let database = Database::connect(db_config).await?;

        const MIGRATIONS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../migrations");
        database.run_migrations(MIGRATIONS_DIR).await?;

        let business_queries = BusinessQueries::new(database.clone());
        let api_key_queries = ApiKeyQueries::new(database.clone());

        Ok(AppState {
            database,
            business_queries,
            api_key_queries,
        })
    }
}
