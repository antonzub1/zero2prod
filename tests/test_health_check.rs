use std::net::TcpListener;
use serde_json::json;

use sqlx::PgPool;
use zero2prod::routes::User;
use zero2prod::startup::run;

pub struct TestApp {
    pub address: String,
    pub pool: PgPool
}

async fn spawn_app(pool: PgPool) -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind a random port.");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener, pool.clone()).expect("Failed to bind an address.");

    tokio::spawn(server);
    let address = format!("http://127.0.0.1:{}", port);

    TestApp {
        address,
        pool: pool
    }
}

#[sqlx::test]
async fn test_health_check_ok(pool: PgPool) {
    let app = spawn_app(pool).await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to send a request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[sqlx::test]
async fn test_subscribe_ok(pool: PgPool) {
    let app = spawn_app(pool).await;

    let client = reqwest::Client::new();

    let payload = User {
        name: "Ursula Le Guin".into(),
        email: "ursula_le_guin@gmail.com".into()
    };

    let response = client
        .post(format!("{}/subscribe", app.address))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .expect("Failed to send a request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("select email, name from subscriptions",)
        .fetch_one(&app.pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, payload.email);
    assert_eq!(saved.name, payload.name);
}


#[sqlx::test]
async fn test_subscribe_fail(pool: PgPool) {
    let app = spawn_app(pool).await;

    let client = reqwest::Client::new();

    let payload = json!({});
    let response = client
        .post(format!("{}/subscribe", app.address))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .expect("Failed to send a request.");

    assert_eq!(400, response.status().as_u16());
}

