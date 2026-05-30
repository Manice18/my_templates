use axum::{Router, routing::post};

use crate::handlers::create_business;
use crate::state::AppState;

pub fn business_router() -> Router<AppState> {
    Router::new().route("/", post(create_business))
}
