use std::{error::Error, net::TcpListener};

// `cargo expand --test health_check`
#[tokio::test]
async fn health_check_succeeds() {
    // start web server
    let local_addrs = spawn_app().expect("failed to spawn App");

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", local_addrs))
        .send()
        .await
        .expect("Failed to execute GET  request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> Result<String, Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:0")?;

    let socket_addr = listener.local_addr().map_err(|e| e.into());
    tokio::spawn(z2p_mailerr::run(listener)?);

    socket_addr.map(|p| format!("http://127.0.0.1:{}", p.port()))
}

#[tokio::test]
async fn subscribe_returns_200_for_a_valid_form_data() {
    let local_addr = spawn_app().expect("Failed to spawn App");
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/subscriptions", local_addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // StatusCode: 200
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let local_addr = spawn_app().expect("Failed to spawn App");
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    let client = reqwest::Client::new();

    for (body, error_msg) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", local_addr))
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
