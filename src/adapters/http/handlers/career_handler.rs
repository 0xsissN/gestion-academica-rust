use crate::adapters::http::responses::api_response::ApiResponse;
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Career {
    pub id: i32,
    pub name: String,
    pub is_active: bool,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateCareer {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCareer {
    pub name: Option<String>,
    pub is_active: Option<bool>,
}

pub async fn get_careers(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<ApiResponse<Vec<Career>>>), StatusCode> {
    let careers = sqlx::query_as::<_, Career>("SELECT * FROM career")
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ApiResponse::ok(careers))
}

pub async fn get_career_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<ApiResponse<Career>>), StatusCode> {
    let career =
        sqlx::query_as::<_, Career>("SELECT * FROM career WHERE id = $1 AND is_active = true")
            .bind(&id)
            .fetch_optional(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match career {
        Some(career) => Ok(ApiResponse::ok(career)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_career(
    State(state): State<AppState>,
    Json(payload): Json<CreateCareer>,
) -> Result<(StatusCode, Json<ApiResponse<Career>>), StatusCode> {
    let career = sqlx::query_as::<_, Career>("INSERT INTO career (name) VALUES ($1) RETURNING *")
        .bind(&payload.name)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ApiResponse::created(career))
}

pub async fn update_career(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateCareer>,
) -> Result<(StatusCode, Json<ApiResponse<Career>>), StatusCode> {
    let result = sqlx::query_as::<_, Career>(
        "UPDATE career SET name = COALESCE($1, name), is_active = COALESCE($2, is_active) WHERE id = $3 RETURNING *", 
    )
    .bind(&payload.name)
    .bind(&payload.is_active)
    .bind(&id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match result {
        Some(career) => Ok(ApiResponse::ok(career)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn delete_career(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<ApiResponse<String>>), StatusCode> {
    let result = sqlx::query("UPDATE career SET is_active = false WHERE id = $1")
        .bind(&id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(ApiResponse::ok("Career deleted successfully".to_string()))
}
