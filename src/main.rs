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
    let connection_pool = PgPool::connect(&connection_url.expose_secret())
        .await
        .expect("failed to connect to Database!");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config_settings.application_port))?;
    run(listener, connection_pool)?.await
}
