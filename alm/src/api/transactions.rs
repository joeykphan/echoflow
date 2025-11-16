use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db::{models::Transaction, DbPool},
    utils::{auth::AuthUser, AppError},
};

pub fn routes(pool: DbPool) -> Router {
    Router::new()
        .route("/", get(list_transactions).post(create_transaction))
        .route(
            "/:id",
            get(get_transaction)
                .put(update_transaction)
                .delete(delete_transaction),
        )
        .with_state(pool)
}

#[derive(Deserialize)]
struct TransactionQuery {
    account_id: Option<Uuid>,
    category_id: Option<Uuid>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    uncategorized: Option<bool>,
}

async fn list_transactions(
    AuthUser { user_id }: AuthUser,
    State(pool): State<DbPool>,
    Query(query): Query<TransactionQuery>,
) -> Result<Json<Vec<Transaction>>, AppError> {
    let mut sql = String::from(
        "SELECT t.* FROM transactions t 
         JOIN accounts a ON t.account_id = a.id 
         WHERE a.user_id = $1",
    );

    let mut param_count = 1;

    if query.account_id.is_some() {
        param_count += 1;
        sql.push_str(&format!(" AND t.account_id = ${}", param_count));
    }

    if query.category_id.is_some() {
        param_count += 1;
        sql.push_str(&format!(" AND t.category_id = ${}", param_count));
    }

    if query.uncategorized == Some(true) {
        sql.push_str(" AND t.category_id IS NULL");
    }

    if query.start_date.is_some() {
        param_count += 1;
        sql.push_str(&format!(" AND t.date >= ${}", param_count));
    }

    if query.end_date.is_some() {
        param_count += 1;
        sql.push_str(&format!(" AND t.date <= ${}", param_count));
    }

    sql.push_str(" ORDER BY t.date DESC, t.created_at DESC");

    let mut query_builder = sqlx::query_as::<_, Transaction>(&sql).bind(user_id);

    if let Some(account_id) = query.account_id {
        query_builder = query_builder.bind(account_id);
    }
    if let Some(category_id) = query.category_id {
        query_builder = query_builder.bind(category_id);
    }
    if let Some(start_date) = query.start_date {
        query_builder = query_builder.bind(start_date);
    }
    if let Some(end_date) = query.end_date {
        query_builder = query_builder.bind(end_date);
    }

    let transactions = query_builder.fetch_all(&pool).await?;

    Ok(Json(transactions))
}

async fn get_transaction(
    AuthUser { user_id }: AuthUser,
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Transaction>, AppError> {
    let transaction = sqlx::query_as!(
        Transaction,
        "SELECT t.* FROM transactions t 
         JOIN accounts a ON t.account_id = a.id 
         WHERE t.id = $1 AND a.user_id = $2",
        id,
        user_id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(transaction))
}

#[derive(Deserialize)]
struct CreateTransactionRequest {
    account_id: Uuid,
    date: NaiveDate,
    amount: f64,
    description: String,
    category_id: Option<Uuid>,
    merchant_name: Option<String>,
}

async fn create_transaction(
    AuthUser { user_id }: AuthUser,
    State(pool): State<DbPool>,
    Json(payload): Json<CreateTransactionRequest>,
) -> Result<Json<Transaction>, AppError> {
    let account = sqlx::query!(
        "SELECT id FROM accounts WHERE id = $1 AND user_id = $2",
        payload.account_id,
        user_id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::BadRequest("Invalid account".to_string()))?;

    let transaction_id = Uuid::new_v4();

    let transaction = sqlx::query_as!(
        Transaction,
        "INSERT INTO transactions (id, account_id, date, amount, description, category_id, merchant_name, pending) 
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8) 
         RETURNING *",
        transaction_id,
        payload.account_id,
        payload.date,
        payload.amount,
        payload.description,
        payload.category_id,
        payload.merchant_name,
        false
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(transaction))
}

#[derive(Deserialize)]
struct UpdateTransactionRequest {
    category_id: Option<Uuid>,
    description: Option<String>,
    amount: Option<f64>,
}

async fn update_transaction(
    AuthUser { user_id }: AuthUser,
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTransactionRequest>,
) -> Result<Json<Transaction>, AppError> {
    let existing = sqlx::query!(
        "SELECT t.id FROM transactions t 
         JOIN accounts a ON t.account_id = a.id 
         WHERE t.id = $1 AND a.user_id = $2",
        id,
        user_id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;

    if let Some(category_id) = payload.category_id {
        sqlx::query!(
            "UPDATE transactions SET category_id = $1, updated_at = NOW() WHERE id = $2",
            category_id,
            id
        )
        .execute(&pool)
        .await?;
    }

    if let Some(description) = payload.description {
        sqlx::query!(
            "UPDATE transactions SET description = $1, updated_at = NOW() WHERE id = $2",
            description,
            id
        )
        .execute(&pool)
        .await?;
    }

    if let Some(amount) = payload.amount {
        sqlx::query!(
            "UPDATE transactions SET amount = $1, updated_at = NOW() WHERE id = $2",
            amount,
            id
        )
        .execute(&pool)
        .await?;
    }

    let transaction = sqlx::query_as!(Transaction, "SELECT * FROM transactions WHERE id = $1", id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(transaction))
}

async fn delete_transaction(
    AuthUser { user_id }: AuthUser,
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    sqlx::query!(
        "DELETE FROM transactions t 
         USING accounts a 
         WHERE t.account_id = a.id AND t.id = $1 AND a.user_id = $2",
        id,
        user_id
    )
    .execute(&pool)
    .await?;

    Ok(Json(()))
}
