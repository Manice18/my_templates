pub mod v1;

use axum::{
    Router,
    http::{Method, header},
    middleware,
    routing::get,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers::health_check;
use crate::state::AppState;
use shared::{CorrelationIdExt, correlation_middleware};

fn request_uri_for_span<B>(request: &axum::http::Request<B>) -> &str {
    request.uri().path()
}

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::health::health_check,
        crate::handlers::business::create_business,
        crate::handlers::me::get_me,
    ),
    components(
        schemas(
            shared::errors::ErrorResponse,
            shared::types::BusinessCreateResponse,
            shared::types::MeResponse,
            crate::handlers::business::BusinessCreateRequest,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoint"),
        (name = "Business", description = "Business onboarding (no auth)"),
        (name = "Auth", description = "Authenticated endpoints (Bearer API key)"),
    ),
    modifiers(&SecurityAddon),
    info(
        title = "Rust Backend API",
        version = "1.0.0",
        description = "Starter API scaffold — multi-tenant businesses with Bearer API keys.",
    ),
)]
struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("API key")
                        .description(Some(
                            "Business API key from POST /api/v1/business (e.g. sk_live_abcd1234.xxx)",
                        ))
                        .build(),
                ),
            );
        }
    }
}

pub fn create_router(app_state: AppState) -> Router {
    let cors_layer = build_cors_layer();

    let swagger = SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi());

    Router::new()
        .merge(swagger)
        .route("/", get(|| async { "Rust Backend API v1.0.0" }))
        .route("/health", get(health_check))
        .nest("/api/v1", v1::create_v1_router())
        .layer(cors_layer)
        .layer(middleware::from_fn(correlation_middleware))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &axum::extract::Request| {
                let correlation_id = request.correlation_id().inner();
                tracing::info_span!(
                    "http_request",
                    method = %request.method(),
                    uri = %request_uri_for_span(request),
                    correlation_id = %correlation_id,
                )
            }),
        )
        .with_state(app_state)
}

pub fn log_endpoints() {
    tracing::info!("=== API Endpoints (v1.0.0) ===");
    tracing::info!("  GET    / - API info");
    tracing::info!("  GET    /health - Health check");
    tracing::info!("  GET    /swagger-ui - Swagger UI");
    tracing::info!("  POST   /api/v1/business - Create business (returns API key)");
    tracing::info!("  GET    /api/v1/me - Authenticated business context (Bearer API key)");
}

fn build_cors_layer() -> CorsLayer {
    let allowed_origins: Vec<String> = std::env::var("CORS_ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "*".to_string())
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string)
        .collect();

    let use_wildcard = allowed_origins.iter().any(|o| o == "*");

    let cors = if use_wildcard {
        tracing::info!("CORS configured with wildcard — allowing all origins");
        CorsLayer::new().allow_origin(tower_http::cors::Any)
    } else {
        let origins: Vec<axum::http::HeaderValue> = allowed_origins
            .iter()
            .filter_map(|origin| {
                origin
                    .parse()
                    .map_err(|e| {
                        tracing::warn!("Invalid CORS origin '{origin}': {e}");
                    })
                    .ok()
            })
            .collect();
        CorsLayer::new().allow_origin(origins)
    };

    cors.allow_methods([
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::DELETE,
        Method::PATCH,
        Method::OPTIONS,
    ])
    .allow_headers([
        header::AUTHORIZATION,
        header::CONTENT_TYPE,
        header::ACCEPT,
        header::ORIGIN,
        shared::CORRELATION_ID_HEADER
            .parse()
            .expect("valid header name"),
    ])
    .expose_headers([
        header::CONTENT_TYPE,
        header::AUTHORIZATION,
        shared::CORRELATION_ID_HEADER
            .parse()
            .expect("valid header name"),
    ])
    .max_age(std::time::Duration::from_secs(3600))
}
