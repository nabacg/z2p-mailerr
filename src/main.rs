use std::{net::TcpListener, time::Duration};

use sqlx::postgres::PgPoolOptions;

use z2p_mailerr::{
    configuration,
    startup::run,
    telemetry::{get_tracing_subscriber, init_tracing},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_tracing_subscriber("z2p_mailerr".into(), "info".into(), std::io::stdout);
    init_tracing(subscriber);
    let config_settings =
        configuration::get_configuration().expect("failed to read configuration!");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(2))
        .connect_lazy_with(config_settings.database.with_db());

    let listener = TcpListener::bind(format!(
        "{}:{}",
        config_settings.application.host, config_settings.application.port
    ))?;
    run(listener, connection_pool)?.await
}
