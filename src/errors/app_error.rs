use crate::adapters::http::responses::api_response::ApiResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum AppError {
    NotFound,
    BadRequest(String),
    DatabaseError,
    InternalServerError,
}

impl From<sqlx::Error> for AppError {
    fn from(_: sqlx::Error) -> Self {
        AppError::DatabaseError
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => {
                ApiResponse::<()>::error("Resource not found", StatusCode::NOT_FOUND)
                    .into_response()
            }

            AppError::BadRequest(msg) => {
                ApiResponse::<()>::error(&msg, StatusCode::BAD_REQUEST).into_response()
            }

            AppError::DatabaseError => {
                ApiResponse::<()>::error("Database error", StatusCode::INTERNAL_SERVER_ERROR)
                    .into_response()
            }

            AppError::InternalServerError => {
                ApiResponse::<()>::error("Internal server error", StatusCode::INTERNAL_SERVER_ERROR)
                    .into_response()
            }
        }
    }
}
