use std::net::TcpListener;

use env_logger::Env;
use sqlx::PgPool;
use z2p_mailerr::{configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // set Logger, if RUST_LOG is not set, default to `info`
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config_settings =
        configuration::get_configuration().expect("failed to read configuration!");
    let connection_url = config_settings.database.connection_string();
    let connection_pool = PgPool::connect(&connection_url)
        .await
        .expect("failed to connect to Database!");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config_settings.application_port))?;
    run(listener, connection_pool)?.await
}
