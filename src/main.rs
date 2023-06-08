use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::PgPool;

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
    let connection_url = config_settings.database.connection_string();
    let connection_pool = PgPool::connect_lazy(connection_url.expose_secret())
        .expect("failed to create Postgres connection pool!");

    let listener = TcpListener::bind(format!(
        "{}:{}",
        config_settings.application.host, config_settings.application.port
    ))?;
    run(listener, connection_pool)?.await
}
