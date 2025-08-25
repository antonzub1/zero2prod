use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::config::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", config.port);
    let listener = TcpListener::bind(address)?;
    let pool = PgPool::connect(
        &config.database.connection_string()
    )
        .await
        .expect("Failed to connect to database.");

    println!("Running at: {:?}", listener);
    run(listener, pool)?.await
}
