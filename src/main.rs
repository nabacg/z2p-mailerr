use std::net::TcpListener;

use sqlx::{Connection, PgConnection, PgPool};
use z2p_mailerr::{configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config_settings =
        configuration::get_configuration().expect("failed to read configuration!");
    let connection_url = config_settings.database.connection_string();
    let connection_pool = PgPool::connect(&connection_url)
        .await
        .expect("failed to connect to Database!");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config_settings.application_port))?;
    run(listener, connection_pool)?.await
}
