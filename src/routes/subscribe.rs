use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct UserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserResponse {
    user_id: Uuid
}

#[tracing::instrument(
    name="Adding a new subscriber.",
    skip(user, pool),
    fields(
        name=%user.name,
        email=%user.email
    )
)]
pub async fn subscribe(user: Json<UserRequest>, pool: Data<PgPool>) -> impl Responder {
    match create_subscriber(&pool, &user).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Saving new user to the database.", skip(user, pool))]
pub async fn create_subscriber(pool: &PgPool, user: &UserRequest) -> Result<UserResponse, sqlx::Error> {
    let record = sqlx::query!(
        r#"insert into subscriptions (id, email, name, subscribed_at) values ($1, $2, $3, $4) returning id"#,
        Uuid::new_v4(),
        user.email,
        user.name,
        Utc::now()
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute a query: {:?}", e);
        e
    })?;
    Ok(UserResponse {
        user_id: record.id
    })
}
