use axum::{
    Json,
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use tracing::{error, warn};
use uuid::Uuid;

use crate::errors::{AppError, ErrorResponse};

pub const CORRELATION_ID_HEADER: &str = "x-correlation-id";

#[derive(Clone)]
pub struct CorrelationId(pub Uuid);

impl CorrelationId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_headers(headers: &HeaderMap) -> Self {
        headers
            .get(CORRELATION_ID_HEADER)
            .and_then(|value| value.to_str().ok())
            .and_then(|s| Uuid::parse_str(s).ok())
            .map(Self)
            .unwrap_or_default()
    }

    pub fn inner(&self) -> Uuid {
        self.0
    }
}

impl Default for CorrelationId {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn correlation_middleware(mut request: Request, next: Next) -> Response {
    let correlation_id = CorrelationId::from_headers(request.headers());

    request.extensions_mut().insert(correlation_id.clone());
    let mut response = next.run(request).await;

    response.headers_mut().insert(
        CORRELATION_ID_HEADER,
        correlation_id.0.to_string().parse().unwrap(),
    );

    response
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code =
            StatusCode::from_u16(self.http_status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        let correlation_id = Uuid::new_v4();

        match status_code {
            StatusCode::INTERNAL_SERVER_ERROR => {
                error!(
                    correlation_id = %correlation_id,
                    error_code = self.error_code(),
                    error = %self,
                    "Internal server error occurred"
                );
            }
            StatusCode::BAD_REQUEST | StatusCode::UNAUTHORIZED | StatusCode::NOT_FOUND => {
                warn!(
                    correlation_id = %correlation_id,
                    error_code = self.error_code(),
                    error = %self,
                    "Client error occurred"
                );
            }
            _ => {
                warn!(
                    correlation_id = %correlation_id,
                    error_code = self.error_code(),
                    error = %self,
                    "Service error occurred"
                );
            }
        }

        let error_response = ErrorResponse::with_correlation_id(&self, correlation_id);

        let mut response = Json(error_response).into_response();
        *response.status_mut() = status_code;

        response.headers_mut().insert(
            CORRELATION_ID_HEADER,
            correlation_id.to_string().parse().unwrap(),
        );

        response
    }
}

pub trait CorrelationIdExt {
    fn correlation_id(&self) -> CorrelationId;
}

impl CorrelationIdExt for axum::extract::Request {
    fn correlation_id(&self) -> CorrelationId {
        self.extensions()
            .get::<CorrelationId>()
            .cloned()
            .unwrap_or_default()
    }
}

pub fn error_response_with_context(
    error: AppError,
    correlation_id: Option<CorrelationId>,
) -> Response {
    let correlation_id = correlation_id.map(|c| c.0).unwrap_or_else(Uuid::new_v4);
    let status_code =
        StatusCode::from_u16(error.http_status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    match status_code {
        StatusCode::INTERNAL_SERVER_ERROR => {
            error!(
                correlation_id = %correlation_id,
                error_code = error.error_code(),
                error = %error,
                "Internal server error occurred"
            );
        }
        StatusCode::BAD_REQUEST | StatusCode::UNAUTHORIZED | StatusCode::NOT_FOUND => {
            warn!(
                correlation_id = %correlation_id,
                error_code = error.error_code(),
                error = %error,
                "Client error occurred"
            );
        }
        _ => {
            warn!(
                correlation_id = %correlation_id,
                error_code = error.error_code(),
                error = %error,
                "Service error occurred"
            );
        }
    }

    let error_response = ErrorResponse::with_correlation_id(&error, correlation_id);

    let mut response = Json(error_response).into_response();
    *response.status_mut() = status_code;

    response.headers_mut().insert(
        CORRELATION_ID_HEADER,
        correlation_id.to_string().parse().unwrap(),
    );

    response
}
