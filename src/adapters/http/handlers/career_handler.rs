use crate::adapters::http::dto::career_dto::*;
use crate::adapters::http::responses::api_response::ApiResponse;
use crate::domain::models::career::Career;
use crate::errors::app_error::AppError;
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
};

pub async fn get_careers(
    State(state): State<AppState>,
) -> Result<ApiResponse<Vec<Career>>, AppError> {
    let careers = sqlx::query_as::<_, Career>("SELECT * FROM career WHERE is_active = true")
        .fetch_all(&state.db)
        .await?;

    Ok(ApiResponse::ok(careers))
}

pub async fn get_career_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<Career>, AppError> {
    let career =
        sqlx::query_as::<_, Career>("SELECT * FROM career WHERE id = $1 AND is_active = true")
            .bind(&id)
            .fetch_optional(&state.db)
            .await?;

    match career {
        Some(career) => Ok(ApiResponse::ok(career)),
        None => Err(AppError::NotFound),
    }
}

pub async fn create_career(
    State(state): State<AppState>,
    Json(payload): Json<CreateCareer>,
) -> Result<ApiResponse<Career>, AppError> {
    let career = sqlx::query_as::<_, Career>("INSERT INTO career (name) VALUES ($1) RETURNING *")
        .bind(&payload.name)
        .fetch_one(&state.db)
        .await?;

    Ok(ApiResponse::created(career))
}

pub async fn update_career(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateCareer>,
) -> Result<ApiResponse<Career>, AppError> {
    let result = sqlx::query_as::<_, Career>(
        "UPDATE career SET name = COALESCE($1, name), is_active = COALESCE($2, is_active) WHERE id = $3 RETURNING *", 
    )
    .bind(&payload.name)
    .bind(&payload.is_active)
    .bind(&id)
    .fetch_optional(&state.db)
    .await?;

    match result {
        Some(career) => Ok(ApiResponse::ok(career)),
        None => Err(AppError::NotFound),
    }
}

pub async fn delete_career(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<()>, AppError> {
    let result = sqlx::query("UPDATE career SET is_active = false WHERE id = $1")
        .bind(&id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(ApiResponse::no_content())
}
