use axum::{Json, extract::State};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::config::get_api_prefix;
use crate::state::AppState;
use shared::types::BusinessCreateResponse;
use shared::{AppError, Result};

#[derive(Debug, Deserialize, ToSchema)]
pub struct BusinessCreateRequest {
    pub name: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/business",
    request_body = BusinessCreateRequest,
    responses(
        (status = 200, description = "Business created with API key", body = BusinessCreateResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Business",
)]
pub async fn create_business(
    State(state): State<AppState>,
    Json(body): Json<BusinessCreateRequest>,
) -> Result<Json<BusinessCreateResponse>> {
    let name = body.name.trim();

    if name.is_empty() {
        return Err(AppError::BadRequest("name must not be empty".to_string()));
    }

    let response = state
        .business_queries
        .create_business_with_api_key(name, &get_api_prefix())
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create business: {e}")))?;

    Ok(Json(response))
}
