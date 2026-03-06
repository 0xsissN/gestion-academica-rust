use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Course {
    pub id: i32,
    pub name: String,
    pub is_active: bool,
    pub credits: f32,
    pub created_at: chrono::NaiveDateTime,
    pub career_id: i32,
}
