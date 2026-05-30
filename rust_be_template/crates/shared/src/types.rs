use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct Business {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

/// Returned once when a business is created. The raw API key is not stored.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BusinessCreateResponse {
    pub name: String,
    pub api_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MeResponse {
    pub business_id: Uuid,
}
