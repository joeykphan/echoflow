use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db::DbPool,
    utils::{auth::create_jwt, AppError},
};

pub fn routes(pool: DbPool) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(pool)
}

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    token: String,
    user_id: String,
}

async fn register(
    State(pool): State<DbPool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let password_hash = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let user_id = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO users (id, email, password_hash) VALUES ($1, $2, $3)",
        user_id,
        payload.email,
        password_hash
    )
    .execute(&pool)
    .await?;

    let token = create_jwt(user_id).map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(AuthResponse {
        token,
        user_id: user_id.to_string(),
    }))
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

async fn login(
    State(pool): State<DbPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = sqlx::query_as!(
        crate::db::models::User,
        "SELECT * FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::Unauthorized)?;

    let valid = bcrypt::verify(&payload.password, &user.password_hash)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    if !valid {
        return Err(AppError::Unauthorized);
    }

    let token = create_jwt(user.id).map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(AuthResponse {
        token,
        user_id: user.id.to_string(),
    }))
}
