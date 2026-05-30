use anyhow::Result;
use tokio::net::TcpListener;

use api::{
    config::get_database_url,
    routes::{create_router, log_endpoints},
    state::AppState,
};

#[tokio::main]
async fn main() -> Result<()> {
    shared::init_observability(shared::ObservabilityConfig::api())?;

    let database_url = get_database_url();
    let app_state = AppState::new(&database_url).await?;

    log_endpoints();

    let router = create_router(app_state);

    let addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("Server listening on http://{addr}");
    axum::serve(listener, router).await?;

    Ok(())
}
