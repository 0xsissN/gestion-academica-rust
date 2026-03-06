use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Career {
    pub id: i32,
    pub name: String,
    pub is_active: bool,
    pub created_at: chrono::NaiveDateTime,
}
