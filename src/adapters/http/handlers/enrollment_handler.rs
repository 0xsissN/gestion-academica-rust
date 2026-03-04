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

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Enrollment {
    pub id: i32,
    pub enrollment_date: chrono::NaiveDateTime,
    pub grade: f32,
    pub student_id: Uuid,
    pub course_id: i32,
}

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

pub async fn get_enrollments(
    State(state): State<AppState>,
) -> Result<ApiResponse<Vec<Enrollment>>, AppError> {
    let enrollments = sqlx::query_as::<_, Enrollment>("SELECT * FROM enrollment")
        .fetch_all(&state.db)
        .await?;

    Ok(ApiResponse::ok(enrollments))
}

pub async fn get_enrollment_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<Enrollment>, AppError> {
    let enrollment = sqlx::query_as::<_, Enrollment>("SELECT * FROM enrollment WHERE id = $1")
        .bind(&id)
        .fetch_optional(&state.db)
        .await?;

    match enrollment {
        Some(enrollment) => Ok(ApiResponse::ok(enrollment)),
        None => Err(AppError::NotFound),
    }
}

pub async fn create_enrollment(
    State(state): State<AppState>,
    Json(payload): Json<CreateEnrollment>,
) -> Result<ApiResponse<Enrollment>, AppError> {
    let enrollment = sqlx::query_as::<_, Enrollment>(
        "INSERT INTO enrollment (student_id, course_id) VALUES ($1, $2) RETURNING *",
    )
    .bind(&payload.student_id)
    .bind(&payload.course_id)
    .fetch_one(&state.db)
    .await?;

    Ok(ApiResponse::created(enrollment))
}

pub async fn update_enrollment(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateEnrollment>,
) -> Result<ApiResponse<Enrollment>, AppError> {
    let result = sqlx::query_as::<_, Enrollment>("UPDATE enrollment SET grade = COALESCE($1, grade), student_id = COALESCE($2, student_id), course_id = COALESCE($3, course_id) WHERE id = $4 RETURNING *")
        .bind(&payload.grade)
        .bind(&payload.student_id)
        .bind(&payload.course_id)
        .bind(&id)
        .fetch_optional(&state.db)
        .await?;

    match result {
        Some(enrollment) => Ok(ApiResponse::ok(enrollment)),
        None => Err(AppError::NotFound),
    }
}

pub async fn delete_enrollment(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<()>, AppError> {
    let result = sqlx::query("UPDATE enrollment SET is_active = false WHERE id = $1")
        .bind(&id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(ApiResponse::no_content())
}
