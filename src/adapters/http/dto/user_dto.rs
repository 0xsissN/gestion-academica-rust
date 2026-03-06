use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub role_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_active: Option<bool>,
    pub role_id: Option<i32>,
}
