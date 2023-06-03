use sqlx::SqlitePool;
use std::net::TcpListener;

use home::configuration::get_configuration;
use home::startup::run;
use home::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("home".into(), "info".into(), std::io::stdout);

    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");

    let connection_pool = SqlitePool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Sqlite");

    let address = format!("localhost:{}", configuration.application_port);
    let error_message = format!("Failed to bind to {}", &address);
    let listener = TcpListener::bind(address).expect(&error_message);
    run(listener, connection_pool)?.await
}
