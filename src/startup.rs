use std::net::TcpListener;

use crate::routes::{get_subscriptions, health_check, subscribe};

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::SqlitePool;

use tracing_actix_web::TracingLogger;

pub fn run(
    listener: TcpListener,
    db_pool: SqlitePool,
) -> std::io::Result<Server> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/subscriptions", web::get().to(get_subscriptions))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
