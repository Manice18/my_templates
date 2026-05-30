use crate::Database;
use crate::api_keys;
use anyhow::{Context, Result, bail};
use uuid::Uuid;

#[derive(Clone)]
pub struct ApiKeyQueries {
    db: Database,
}

impl ApiKeyQueries {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn authenticate(&self, full_key: &str) -> Result<Uuid> {
        let (prefix, _) = full_key.split_once('.').context("invalid API key format")?;

        let row = sqlx::query_as::<_, ApiKeyRow>(
            r#"
            SELECT business_id, key_hash
            FROM api_key
            WHERE key_prefix = $1
              AND revoked_at IS NULL
            "#,
        )
        .bind(prefix)
        .fetch_optional(&self.db.pool)
        .await?
        .context("invalid API key")?;

        let valid = api_keys::verify_api_key(full_key, &row.key_hash)?;

        if !valid {
            bail!("invalid API key");
        }

        Ok(row.business_id)
    }
}

#[derive(sqlx::FromRow)]
struct ApiKeyRow {
    business_id: Uuid,
    key_hash: String,
}
