use crate::adapters::http::dto::course_dto::*;
use crate::adapters::http::responses::api_response::ApiResponse;
use crate::domain::models::course::Course;
use crate::errors::app_error::AppError;
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
};

pub async fn get_courses(
    State(state): State<AppState>,
) -> Result<ApiResponse<Vec<Course>>, AppError> {
    let courses = sqlx::query_as::<_, Course>("SELECT * FROM course")
        .fetch_all(&state.db)
        .await?;

    Ok(ApiResponse::ok(courses))
}

pub async fn get_course_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<Course>, AppError> {
    let course = sqlx::query_as::<_, Course>("SELECT * FROM course WHERE id = $1")
        .bind(&id)
        .fetch_optional(&state.db)
        .await?;

    match course {
        Some(course) => Ok(ApiResponse::ok(course)),
        None => Err(AppError::NotFound),
    }
}

pub async fn create_course(
    State(state): State<AppState>,
    Json(payload): Json<CreateCourse>,
) -> Result<ApiResponse<Course>, AppError> {
    let course = sqlx::query_as::<_, Course>(
        "INSERT INTO course (name, credits, career_id) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(&payload.name)
    .bind(&payload.credits)
    .bind(&payload.career_id)
    .fetch_one(&state.db)
    .await?;

    Ok(ApiResponse::created(course))
}

pub async fn update_course(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateCourse>,
) -> Result<ApiResponse<Course>, AppError> {
    let result = sqlx::query_as::<_, Course>(
        "UPDATE course SET name = COALESCE($1, name), is_active = COALESCE($2, is_active), credits = COALESCE($3, credits), career_id = COALESCE($4, career_id) WHERE id = $5 RETURNING *",
    )
    .bind(&payload.name)
    .bind(&payload.is_active)
    .bind(&payload.credits)
    .bind(&payload.career_id)
    .bind(&id)
    .fetch_optional(&state.db)
    .await?;

    match result {
        Some(course) => Ok(ApiResponse::ok(course)),
        None => Err(AppError::NotFound),
    }
}

pub async fn delete_course(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<()>, AppError> {
    let result = sqlx::query("UPDATE course SET is_active = false WHERE id = $1")
        .bind(&id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(ApiResponse::no_content())
}
