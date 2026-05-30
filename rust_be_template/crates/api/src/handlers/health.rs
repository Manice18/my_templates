use axum::{extract::State, http::StatusCode, response::Json};
use serde_json::{Value, json};

use crate::state::AppState;
use shared::{CorrelationId, Result};

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health check response", body = Value),
        (status = 503, description = "Health check failed", body = Value)
    ),
    tag = "Health"
)]
pub async fn health_check(
    State(state): State<AppState>,
    correlation_id: Option<axum::Extension<CorrelationId>>,
) -> Result<(StatusCode, Json<Value>)> {
    let correlation_id = correlation_id
        .map(|ext| ext.0.inner())
        .unwrap_or_else(uuid::Uuid::new_v4);

    tracing::info!(
        correlation_id = %correlation_id,
        "Health check requested"
    );

    let db_ok = match state.database.health_check().await {
        Ok(_) => {
            tracing::debug!(
                correlation_id = %correlation_id,
                "Database health check passed"
            );
            true
        }
        Err(e) => {
            tracing::error!(
                correlation_id = %correlation_id,
                error = %e,
                "Database health check failed"
            );
            false
        }
    };

    let status_code = if db_ok {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    Ok((
        status_code,
        Json(json!({
            "status": if db_ok { "healthy" } else { "unhealthy" },
            "database": if db_ok { "connected" } else { "disconnected" },
            "correlation_id": correlation_id,
            "timestamp": chrono::Utc::now().to_rfc3339()
        })),
    ))
}
