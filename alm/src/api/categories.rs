use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    db::{models::Category, DbPool},
    utils::{auth::AuthUser, AppError},
};

pub fn routes(pool: DbPool) -> Router {
    Router::new()
        .route("/", get(list_categories).post(create_category))
        .route(
            "/:id",
            get(get_category)
                .put(update_category)
                .delete(delete_category),
        )
        .with_state(pool)
}

async fn list_categories(
    AuthUser { user_id }: AuthUser,
    State(pool): State<DbPool>,
) -> Result<Json<Vec<Category>>, AppError> {
    let categories = sqlx::query_as!(
        Category,
        "SELECT * FROM categories WHERE user_id = $1 OR is_default = true ORDER BY name",
        user_id
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(categories))
}

async fn get_category(
    AuthUser { user_id }: AuthUser,
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Category>, AppError> {
    let category = sqlx::query_as!(
        Category,
        "SELECT * FROM categories WHERE id = $1 AND (user_id = $2 OR is_default = true)",
        id,
        user_id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(category))
}

#[derive(Deserialize)]
struct CreateCategoryRequest {
    name: String,
    category_type: String,
    color: String,
    icon: Option<String>,
}

async fn create_category(
    AuthUser { user_id }: AuthUser,
    State(pool): State<DbPool>,
    Json(payload): Json<CreateCategoryRequest>,
) -> Result<Json<Category>, AppError> {
    let category_id = Uuid::new_v4();

    let category = sqlx::query_as!(
        Category,
        "INSERT INTO categories (id, user_id, name, category_type, color, icon, is_default) 
         VALUES ($1, $2, $3, $4, $5, $6, $7) 
         RETURNING *",
        category_id,
        user_id,
        payload.name,
        payload.category_type,
        payload.color,
        payload.icon,
        false
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(category))
}

#[derive(Deserialize)]
struct UpdateCategoryRequest {
    name: Option<String>,
    color: Option<String>,
    icon: Option<String>,
}

async fn update_category(
    AuthUser { user_id }: AuthUser,
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> Result<Json<Category>, AppError> {
    let existing = sqlx::query!(
        "SELECT id FROM categories WHERE id = $1 AND user_id = $2 AND is_default = false",
        id,
        user_id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;

    if let Some(name) = payload.name {
        sqlx::query!("UPDATE categories SET name = $1 WHERE id = $2", name, id)
            .execute(&pool)
            .await?;
    }

    if let Some(color) = payload.color {
        sqlx::query!("UPDATE categories SET color = $1 WHERE id = $2", color, id)
            .execute(&pool)
            .await?;
    }

    if let Some(icon) = payload.icon {
        sqlx::query!("UPDATE categories SET icon = $1 WHERE id = $2", icon, id)
            .execute(&pool)
            .await?;
    }

    let category = sqlx::query_as!(Category, "SELECT * FROM categories WHERE id = $1", id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(category))
}

async fn delete_category(
    AuthUser { user_id }: AuthUser,
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    sqlx::query!(
        "DELETE FROM categories WHERE id = $1 AND user_id = $2 AND is_default = false",
        id,
        user_id
    )
    .execute(&pool)
    .await?;

    Ok(Json(()))
}
