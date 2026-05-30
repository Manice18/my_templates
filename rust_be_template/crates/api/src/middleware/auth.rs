use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header::AUTHORIZATION, request::Parts},
};
use uuid::Uuid;

use crate::state::AppState;
use shared::AppError;

#[derive(Debug, Clone, Copy)]
pub struct AuthenticatedBusiness {
    pub business_id: Uuid,
}

impl<S> FromRequestParts<S> for AuthenticatedBusiness
where
    AppState: axum::extract::FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        let token = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| {
                AppError::Unauthorized(
                    "Missing or invalid Authorization header. Use: Bearer <api_key>".to_string(),
                )
            })?;

        let business_id = app_state
            .api_key_queries
            .authenticate(token)
            .await
            .map_err(|_| AppError::Unauthorized("Invalid API key".to_string()))?;

        Ok(Self { business_id })
    }
}
