use std::net::TcpListener;

use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use sqlx::PgPool;


pub fn run(
    listener: TcpListener,
    connection: PgPool
) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(crate::routes::healthcheck))
            .route("/subscribe", web::post().to(crate::routes::subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
