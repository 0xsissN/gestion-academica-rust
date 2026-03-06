use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub role_id: i32,
}
