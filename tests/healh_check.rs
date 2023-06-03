use std::{error::Error, net::TcpListener};

use sqlx::{Connection, PgConnection, PgPool};
use z2p_mailerr::configuration::{self, get_configuration};

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port!");

    let socket_addr = listener.local_addr().expect("Failed to get local_addrs");

    let config_settings = configuration::get_configuration().expect("Failed to read configuration");
    let connection_url = config_settings.database.connection_string();

    let connection = PgPool::connect(&connection_url)
        .await
        .expect("Failed to create DB connection pool");
    tokio::spawn(
        z2p_mailerr::startup::run(listener, connection.clone()).expect("failed to run App"),
    );
    TestApp {
        db_pool: connection,
        address: format!("http://127.0.0.1:{}", socket_addr.port()),
    }
}

// `cargo expand --test health_check`
#[tokio::test]
async fn health_check_succeeds() {
    // start web server
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute GET  request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_a_valid_form_data() {
    // start web server
    let app = spawn_app().await;

    let saved_sub = sqlx::query!("delete from public.subscriptions",)
        .execute(&app.db_pool)
        .await
        .expect("failed to delete all subscriptions");

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/subscriptions", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // StatusCode: 200
    assert_eq!(200, response.status().as_u16());

    let saved_sub = sqlx::query!("select * from public.subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("failed to query subscriptions");

    assert_eq!("le guin", saved_sub.name);
    assert_eq!("ursula_le_guin@gmail.com", saved_sub.email);
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    // start web server
    let app = spawn_app().await;

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    let client = reqwest::Client::new();

    for (body, error_msg) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_msg
        );
    }
}
