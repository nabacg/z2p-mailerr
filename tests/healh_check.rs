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
