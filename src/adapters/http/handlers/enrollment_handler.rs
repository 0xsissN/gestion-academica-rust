use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
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
) -> Result<Json<Vec<Enrollment>>, StatusCode> {
    let enrollments = sqlx::query_as::<_, Enrollment>("SELECT * FROM enrollment")
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(enrollments))
}

pub async fn get_enrollment_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Enrollment>, StatusCode> {
    let enrollment = sqlx::query_as::<_, Enrollment>("SELECT * FROM enrollment WHERE id = $1")
        .bind(&id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match enrollment {
        Some(enrollment) => Ok(Json(enrollment)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_enrollment(
    State(state): State<AppState>,
    Json(payload): Json<CreateEnrollment>,
) -> Result<(StatusCode, Json<Enrollment>), StatusCode> {
    let enrollment = sqlx::query_as::<_, Enrollment>(
        "INSERT INTO enrollment (student_id, course_id) VALUES ($1, $2) RETURNING *",
    )
    .bind(&payload.student_id)
    .bind(&payload.course_id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(enrollment)))
}

pub async fn update_enrollment(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateEnrollment>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("UPDATE enrollment SET grade = COALESCE($1, grade), student_id = COALESCE($2, student_id), course_id = COALESCE($3, course_id) WHERE id = $4")
        .bind(&payload.grade)
        .bind(&payload.student_id)
        .bind(&payload.course_id)
        .bind(&id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::OK)
}

pub async fn delete_enrollment(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("UPDATE enrollment SET is_active = false WHERE id = $1")
        .bind(&id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::OK)
}
