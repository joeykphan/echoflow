use axum::{
    extract::{Path, State},
    routing::{delete, get},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    db::{models::Account, DbPool},
    utils::{auth::AuthUser, AppError},
};

pub fn routes(pool: DbPool) -> Router {
    Router::new()
        .route("/", get(list_accounts))
        .route("/:id", get(get_account).delete(delete_account))
        .with_state(pool)
}

async fn list_accounts(
    AuthUser { user_id }: AuthUser,
    State(pool): State<DbPool>,
) -> Result<Json<Vec<Account>>, AppError> {
    let accounts = sqlx::query_as!(
        Account,
        "SELECT * FROM accounts WHERE user_id = $1 ORDER BY created_at DESC",
        user_id
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(accounts))
}

async fn get_account(
    AuthUser { user_id }: AuthUser,
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Account>, AppError> {
    let account = sqlx::query_as!(
        Account,
        "SELECT * FROM accounts WHERE id = $1 AND user_id = $2",
        id,
        user_id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(account))
}

async fn delete_account(
    AuthUser { user_id }: AuthUser,
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    sqlx::query!(
        "DELETE FROM accounts WHERE id = $1 AND user_id = $2",
        id,
        user_id
    )
    .execute(&pool)
    .await?;

    Ok(Json(()))
}
