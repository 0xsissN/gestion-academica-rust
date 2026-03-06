use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateCareer {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCareer {
    pub name: Option<String>,
    pub is_active: Option<bool>,
}
