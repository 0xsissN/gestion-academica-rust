use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Enrollment {
    pub id: i32,
    pub enrollment_date: chrono::NaiveDateTime,
    pub grade: f32,
    pub student_id: Uuid,
    pub course_id: i32,
}
