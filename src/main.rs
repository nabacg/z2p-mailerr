use std::net::TcpListener;

use z2p_mailerr::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    run(listener)?.await
}
