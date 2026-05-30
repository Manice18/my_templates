use anyhow::Result;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Clone)]
pub struct ObservabilityConfig {
    pub service_name: String,
    pub json_logging: bool,
    pub log_filter: Option<String>,
}

impl ObservabilityConfig {
    pub fn api() -> Self {
        Self {
            service_name: "api".to_string(),
            json_logging: true,
            log_filter: None,
        }
    }
}

pub fn init_observability(config: ObservabilityConfig) -> Result<()> {
    dotenvy::dotenv().ok();
    init_tracing(&config)?;

    tracing::info!(
        service = %config.service_name,
        "Observability initialized"
    );

    Ok(())
}

fn init_tracing(config: &ObservabilityConfig) -> Result<()> {
    let filter_layer = if let Some(custom_filter) = &config.log_filter {
        EnvFilter::try_new(custom_filter)?
    } else {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!("{}=debug,tower_http=debug,sqlx=info", config.service_name).into()
        })
    };

    let registry = tracing_subscriber::registry().with(filter_layer);

    if config.json_logging {
        let fmt_layer = fmt::layer()
            .with_target(true)
            .with_thread_ids(true)
            .with_level(true)
            .with_file(true)
            .with_line_number(true)
            .json();

        registry.with(fmt_layer).init();
    } else {
        let fmt_layer = fmt::layer()
            .with_target(true)
            .with_thread_ids(true)
            .with_level(true)
            .with_file(true)
            .with_line_number(true);

        registry.with(fmt_layer).init();
    }

    tracing::info!(
        service = %config.service_name,
        log_format = if config.json_logging { "json" } else { "pretty" },
        "Tracing initialized"
    );

    Ok(())
}
