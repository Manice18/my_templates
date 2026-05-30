mod business;
mod me;

use axum::Router;

use crate::state::AppState;

pub fn create_v1_router() -> Router<AppState> {
    Router::new()
        .nest("/business", business::business_router())
        .nest("/me", me::me_router())
}
