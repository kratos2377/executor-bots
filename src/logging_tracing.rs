use opentelemetry::sdk::trace;
use opentelemetry::sdk::Resource;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::EnvFilter;
use anyhow::Error;
use crate::conf::configuration::Configuration;



pub fn initialize_logging_and_tracing<T>(
    package_name: &str,
    package_version: &str,
    env_filter_customizer: T,
) -> anyhow::Result<()>
where
    T: Fn(EnvFilter) -> EnvFilter,
{
    let resource: Resource = axum_tracing_opentelemetry::make_resource(
        package_name.to_string(),
        package_version.to_string(),
    );


    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name(package_name.to_string())
        .with_trace_config(
            trace::config()
                .with_resource(resource)
                .with_sampler(trace::Sampler::AlwaysOn),
        )
        .with_max_packet_size(9216) // Default max UDP packet size on macOs
        .with_auto_split_batch(true)
        .install_batch(opentelemetry::runtime::Tokio)?;

    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    let fmt_layer = tracing_subscriber::fmt::layer().json();

    let subscriber = tracing_subscriber::registry()
        .with(fmt_layer)
        .with(env_filter_customizer(
            EnvFilter::from_default_env().add_directive(
                "axum_tracing_opentelemetry=trace"
                    .parse()
                    .unwrap_or_default(),
            ),
        ))
        .with(otel_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

pub fn init(config: &Configuration) -> Result<(), Error> {
    Ok(initialize_logging_and_tracing(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        |mut e| {
            // Configure root level
            if let Some(root_level) = &config.logging.level.root {
                e = e.add_directive(root_level.parse().unwrap_or_default())
            }

            // Configure specific directives
            for directive in &config.logging.level.directives {
                let directive_string = format!("{}={}", directive.namespace, directive.level);
                e = e.add_directive(directive_string.parse().unwrap_or_default());
            }

            e
        },
    )?)
}