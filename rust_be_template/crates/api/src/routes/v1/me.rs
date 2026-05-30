use axum::{Router, routing::get};

use crate::handlers::get_me;
use crate::state::AppState;

pub fn me_router() -> Router<AppState> {
    Router::new().route("/", get(get_me))
}
