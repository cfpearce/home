use std::net::TcpListener;

use crate::routes::{health_check, subscribe};
use actix_web::middleware::Logger;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::SqlitePool;

pub fn run(listener: TcpListener, db_pool: SqlitePool) -> std::io::Result<Server> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}