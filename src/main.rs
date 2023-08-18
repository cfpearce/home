use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::net::TcpListener;

use home::configuration::get_configuration;
use home::startup::run;
use home::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber =
        get_subscriber("home".into(), "info".into(), std::io::stdout);

    init_subscriber(subscriber);

    let configuration =
        get_configuration().expect("Failed to read configuration");

    let connection_pool = SqlitePoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(&configuration.database.connection_string())
        .expect("Failed to create Sqlite connection pool.");

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    tracing::info!("Starting server on {}", address);
    let error_message = format!("Failed to bind to {}", &address);
    let listener = TcpListener::bind(address).expect(&error_message);
    run(listener, connection_pool)?.await
}
