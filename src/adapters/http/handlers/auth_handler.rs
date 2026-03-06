use crate::adapters::http::dto::auth_dto::LoginRequest;
use crate::adapters::http::responses::api_response::ApiResponse;
use crate::auth::password::verify_password;
use crate::auth::{claims::Claims, jwt::create_jwt};
use crate::domain::models::user::User;
use crate::errors::app_error::AppError;
use crate::state::AppState;
use axum::{Json, extract::State};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<ApiResponse<String>, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM \"user\" WHERE username = $1")
        .bind(&payload.username)
        .fetch_optional(&state.db)
        .await?;

    let user = user.ok_or(AppError::NotFound)?;

    let valid = verify_password(&user.password_hash, &payload.password);

    if !valid {
        return Err(AppError::BadRequest("Invalid credentials".into()));
    }

    let claims = Claims {
        sub: user.id,
        role_id: user.role_id,
        exp: 2000000000,
    };

    let token = create_jwt(claims).map_err(|_| AppError::InternalServerError)?;

    Ok(ApiResponse::ok(token))
}
