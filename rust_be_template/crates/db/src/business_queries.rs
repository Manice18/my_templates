use crate::Database;
use crate::api_keys;
use anyhow::Result;
use shared::types::BusinessCreateResponse;
use uuid::Uuid;

#[derive(Clone)]
pub struct BusinessQueries {
    db: Database,
}

impl BusinessQueries {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create_business_with_api_key(
        &self,
        name: &str,
        key_prefix: &str,
    ) -> Result<BusinessCreateResponse> {
        let business_id = Uuid::new_v4();
        let api_key_id = Uuid::new_v4();
        let (full_key, key_prefix) = api_keys::generate_api_key(key_prefix)?;
        let key_hash = api_keys::hash_api_key(&full_key)?;

        let mut tx = self.db.pool.begin().await?;

        sqlx::query(
            r#"
            INSERT INTO business (id, name)
            VALUES ($1, $2)
            "#,
        )
        .bind(business_id)
        .bind(name)
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            r#"
            INSERT INTO api_key (id, business_id, key_prefix, key_hash)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(api_key_id)
        .bind(business_id)
        .bind(&key_prefix)
        .bind(&key_hash)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(BusinessCreateResponse {
            name: name.to_string(),
            api_key: full_key,
        })
    }
}
