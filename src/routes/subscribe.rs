
use actix_web::{HttpResponse, Responder};
use actix_web::web::{Data, Json};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub email: String,
}

pub async fn subscribe(
    user: Json<User>,
    pool: Data<PgPool>
) -> impl Responder {
    sqlx::query!(
        r#"insert into subscriptions (id, email, name, subscribed_at)
        values ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        user.email,
        user.name,
        Utc::now()
    )
        .execute(pool.get_ref())
        .await
        .expect("Failed to insert to the database");
    HttpResponse::Ok().finish()
}
