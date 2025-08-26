use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::PgPool;
use zero2prod::config::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", config.port);
    let listener = TcpListener::bind(address)?;
    let pool = PgPool::connect(config.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to database.");

    run(listener, pool)?.await
}
