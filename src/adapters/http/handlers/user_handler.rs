use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub role_id: i32,
}

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

pub async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, StatusCode> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM \"user\"")
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}

pub async fn get_user_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM \"user\" WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Json(user))
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let user = sqlx::query_as::<_, User>("INSERT INTO \"user\" (username, password_hash, first_name, last_name, role_id) VALUES ($1, $2, $3, $4, $5) RETURNING *")
        .bind(&payload.username)
        .bind(&payload.password_hash)
        .bind(&payload.first_name)
        .bind(&payload.last_name)
        .bind(&payload.role_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUser>,
) -> Result<StatusCode, StatusCode> {
    let _user = sqlx::query_as::<_, User>("UPDATE \"user\" SET username = $1, first_name = $2, last_name = $3, is_active = $4, role_id = $5 WHERE id = $6 RETURNING *")
        .bind(&payload.username)
        .bind(&payload.first_name)
        .bind(&payload.last_name)
        .bind(&payload.is_active)
        .bind(&payload.role_id)
        .bind(id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query("DELETE FROM \"user\" WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(StatusCode::OK)
}
