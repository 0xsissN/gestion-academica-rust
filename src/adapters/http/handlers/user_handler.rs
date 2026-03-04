use crate::adapters::http::responses::api_response::ApiResponse;
use crate::errors::app_error::AppError;
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
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

pub async fn get_users(State(state): State<AppState>) -> Result<ApiResponse<Vec<User>>, AppError> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM \"user\"")
        .fetch_all(&state.db)
        .await?;

    Ok(ApiResponse::ok(users))
}

pub async fn get_user_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<User>, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM \"user\" WHERE id = $1")
        .bind(&id)
        .fetch_optional(&state.db)
        .await?;

    match user {
        Some(user) => Ok(ApiResponse::ok(user)),
        None => Err(AppError::NotFound),
    }
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<ApiResponse<User>, AppError> {
    let user = sqlx::query_as::<_, User>("INSERT INTO \"user\" (username, password_hash, first_name, last_name, role_id) VALUES ($1, $2, $3, $4, $5) RETURNING *")
        .bind(&payload.username)
        .bind(&payload.password_hash)
        .bind(&payload.first_name)
        .bind(&payload.last_name)
        .bind(&payload.role_id)
        .fetch_one(&state.db)
        .await?;

    Ok(ApiResponse::created(user))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUser>,
) -> Result<ApiResponse<User>, AppError> {
    let result = sqlx::query_as::<_, User>("UPDATE \"user\" SET username = COALESCE($1, username), first_name = COALESCE($2, first_name), last_name = COALESCE($3, last_name), is_active = COALESCE($4, is_active), role_id = COALESCE($5, role_id) WHERE id = $6 RETURNING *")
        .bind(&payload.username)
        .bind(&payload.first_name)
        .bind(&payload.last_name)
        .bind(&payload.is_active)
        .bind(&payload.role_id)
        .bind(&id)
        .fetch_optional(&state.db)
        .await?;

    match result {
        Some(user) => Ok(ApiResponse::ok(user)),
        None => Err(AppError::NotFound),
    }
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<()>, AppError> {
    let result = sqlx::query("UPDATE \"user\" SET is_active = false WHERE id = $1")
        .bind(&id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(ApiResponse::no_content())
}
