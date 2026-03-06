use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateEnrollment {
    pub student_id: Uuid,
    pub course_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEnrollment {
    pub grade: Option<f32>,
    pub student_id: Option<Uuid>,
    pub course_id: Option<i32>,
}
