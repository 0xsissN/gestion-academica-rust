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
pub struct Course {
    pub id: i32,
    pub name: String,
    pub is_active: bool,
    pub credits: f32,
    pub created_at: chrono::NaiveDateTime,
    pub career_id: i32,
}

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

pub async fn get_courses(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<ApiResponse<Vec<Course>>>), StatusCode> {
    let courses = sqlx::query_as::<_, Course>("SELECT * FROM course")
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ApiResponse::ok(courses))
}

pub async fn get_course_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<ApiResponse<Course>>), StatusCode> {
    let course = sqlx::query_as::<_, Course>("SELECT * FROM course WHERE id = $1")
        .bind(&id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match course {
        Some(course) => Ok(ApiResponse::ok(course)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_course(
    State(state): State<AppState>,
    Json(payload): Json<CreateCourse>,
) -> Result<(StatusCode, Json<ApiResponse<Course>>), StatusCode> {
    let course = sqlx::query_as::<_, Course>(
        "INSERT INTO course (name, credits, career_id) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(&payload.name)
    .bind(&payload.credits)
    .bind(&payload.career_id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ApiResponse::created(course))
}

pub async fn update_course(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateCourse>,
) -> Result<(StatusCode, Json<ApiResponse<Course>>), StatusCode> {
    let result = sqlx::query_as::<_, Course>(
        "UPDATE course SET name = COALESCE($1, name), is_active = COALESCE($2, is_active), credits = COALESCE($3, credits), career_id = COALESCE($4, career_id) WHERE id = $5 RETURNING *",
    )
    .bind(&payload.name)
    .bind(&payload.is_active)
    .bind(&payload.credits)
    .bind(&payload.career_id)
    .bind(&id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match result {
        Some(course) => Ok(ApiResponse::ok(course)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn delete_course(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<ApiResponse<String>>), StatusCode> {
    let result = sqlx::query("UPDATE course SET is_active = false WHERE id = $1")
        .bind(&id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(ApiResponse::ok("Course deleted successfully".to_string()))
}
