use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication failed: {0}")]
    Unauthorized(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl AppError {
    pub fn http_status(&self) -> u16 {
        match self {
            AppError::Unauthorized(_) => 401,
            AppError::NotFound(_) => 404,
            AppError::BadRequest(_) => 400,
            AppError::Conflict(_) => 409,
            AppError::Internal(_) => 500,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            AppError::Unauthorized(_) => "UNAUTHORIZED",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::BadRequest(_) => "BAD_REQUEST",
            AppError::Conflict(_) => "CONFLICT",
            AppError::Internal(_) => "INTERNAL_ERROR",
        }
    }

    pub fn public_message(&self) -> String {
        match self.http_status() {
            500 => "Internal server error".to_string(),
            status if (500..600).contains(&status) => "Server error".to_string(),
            _ => self.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
    pub message: String,
    pub correlation_id: Option<Uuid>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ErrorResponse {
    pub fn new(error: &AppError, correlation_id: Option<Uuid>) -> Self {
        Self {
            error: error.error_code().to_string(),
            code: error.error_code().to_string(),
            message: error.public_message(),
            correlation_id,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn with_correlation_id(error: &AppError, correlation_id: Uuid) -> Self {
        Self::new(error, Some(correlation_id))
    }
}
