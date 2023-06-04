use std::net::TcpListener;

use actix_web::{dev::Server, guard, web, App, HttpServer};
use sqlx::PgPool;

use crate::routes::{health_check, subscribe};

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route(
                "/subscriptions",
                web::post()
                    .guard(guard::Header(
                        "Content-Type",
                        "application/x-www-form-urlencoded",
                    ))
                    .to(subscribe),
            )
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
