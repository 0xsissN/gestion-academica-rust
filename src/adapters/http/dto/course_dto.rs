use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateCourse {
    pub name: String,
    pub credits: f32,
    pub career_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCourse {
    pub name: Option<String>,
    pub is_active: Option<bool>,
    pub credits: Option<f32>,
    pub career_id: Option<i32>,
}
