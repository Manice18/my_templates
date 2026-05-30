use axum::Json;

use crate::middleware::auth::AuthenticatedBusiness;
use shared::types::MeResponse;
use shared::Result;

#[utoipa::path(
    get,
    path = "/api/v1/me",
    responses(
        (status = 200, description = "Authenticated business context", body = MeResponse),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "Auth",
)]
pub async fn get_me(auth: AuthenticatedBusiness) -> Result<Json<MeResponse>> {
    Ok(Json(MeResponse {
        business_id: auth.business_id,
    }))
}
