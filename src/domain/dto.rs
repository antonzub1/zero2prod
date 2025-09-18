use crate::domain::UserName;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Deserialize, Serialize)]
pub struct UserRequest {
    pub name: UserName,
    pub email: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub subscribed_at: DateTime<Utc>,
}
