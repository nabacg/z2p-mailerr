use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use z2p_mailerr::configuration::{self, DatabaseSettings};

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub(crate) async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port!");

    let socket_addr = listener.local_addr().expect("Failed to get local_addrs");

    let mut config_settings =
        configuration::get_configuration().expect("Failed to read configuration");
    // override DB name to a random db, to isolate each test
    config_settings.database.database_name = Uuid::new_v4().to_string();

    let connection_pool = setup_db(&config_settings.database).await;
    tokio::spawn(
        z2p_mailerr::startup::run(listener, connection_pool.clone()).expect("failed to run App"),
    );
    TestApp {
        db_pool: connection_pool,
        address: format!("http://127.0.0.1:{}", socket_addr.port()),
    }
}

async fn setup_db(cfg: &DatabaseSettings) -> PgPool {
    let connection_url = cfg.connection_string_without_db();
    let mut connection = PgConnection::connect(&connection_url)
        .await
        .expect("Failed to Connect to postgres to create DB;");
    connection
        .execute(format!(r#"CREATE DATABASE "{}""#, cfg.database_name).as_str())
        .await
        .expect("Failed to create test Database!");

    let connection_url = cfg.connection_string();
    let connection_pool = PgPool::connect(&connection_url)
        .await
        .expect("Failed to create DB connection pool");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to run migrations!");

    connection_pool
}
