use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;
use zero2prod::config::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration.");
    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address)?;
    let pool = PgPoolOptions::new().connect_lazy_with(
        config.database.connection_options()
    );

    Ok(run(listener, pool)?.await?)
}
