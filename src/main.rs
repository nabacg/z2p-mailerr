use std::net::TcpListener;

use sqlx::PgPool;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use z2p_mailerr::{configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Redirect all `log` calls to tracing subscriber below
    LogTracer::init().expect("Failed to set logger");

    // set trace span level using RUST_LOG env var and  default to `info` if not set
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(
        "z2p_mailerr".into(),
        // output formatted spans to STDOUT
        std::io::stdout,
    );

    //setup subscriber
    // `.with` provided layer::SubscriberExt, an extension trait
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber).expect("Failed to set global tracing subscriber");

    let config_settings =
        configuration::get_configuration().expect("failed to read configuration!");
    let connection_url = config_settings.database.connection_string();
    let connection_pool = PgPool::connect(&connection_url)
        .await
        .expect("failed to connect to Database!");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config_settings.application_port))?;
    run(listener, connection_pool)?.await
}
