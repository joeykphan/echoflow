use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db::{models::Budget, DbPool},
    utils::{auth::AuthUser, AppError},
};

pub fn routes(pool: DbPool) -> Router {
    Router::new()
        .route("/", get(list_budgets).post(create_budget))
        .route(
            "/:id",
            get(get_budget).put(update_budget).delete(delete_budget),
        )
        .route("/:id/performance", get(budget_performance))
        .with_state(pool)
}

async fn list_budgets(
    AuthUser { user_id }: AuthUser,
    State(pool): State<DbPool>,
) -> Result<Json<Vec<Budget>>, AppError> {
    let budgets = sqlx::query_as!(
        Budget,
        "SELECT * FROM budgets WHERE user_id = $1 ORDER BY start_date DESC",
        user_id
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(budgets))
}

async fn get_budget(
    AuthUser { user_id }: AuthUser,
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Budget>, AppError> {
    let budget = sqlx::query_as!(
        Budget,
        "SELECT * FROM budgets WHERE id = $1 AND user_id = $2",
        id,
        user_id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(budget))
}

#[derive(Deserialize)]
struct CreateBudgetRequest {
    category_id: Uuid,
    amount: f64,
    period: String,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
}

async fn create_budget(
    AuthUser { user_id }: AuthUser,
    State(pool): State<DbPool>,
    Json(payload): Json<CreateBudgetRequest>,
) -> Result<Json<Budget>, AppError> {
    let budget_id = Uuid::new_v4();

    let budget = sqlx::query_as!(
        Budget,
        "INSERT INTO budgets (id, user_id, category_id, amount, period, start_date, end_date) 
         VALUES ($1, $2, $3, $4, $5, $6, $7) 
         RETURNING *",
        budget_id,
        user_id,
        payload.category_id,
        payload.amount,
        payload.period,
        payload.start_date,
        payload.end_date
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(budget))
}

#[derive(Deserialize)]
struct UpdateBudgetRequest {
    amount: Option<f64>,
    end_date: Option<NaiveDate>,
}

async fn update_budget(
    AuthUser { user_id }: AuthUser,
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateBudgetRequest>,
) -> Result<Json<Budget>, AppError> {
    let existing = sqlx::query!(
        "SELECT id FROM budgets WHERE id = $1 AND user_id = $2",
        id,
        user_id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;

    if let Some(amount) = payload.amount {
        sqlx::query!("UPDATE budgets SET amount = $1 WHERE id = $2", amount, id)
            .execute(&pool)
            .await?;
    }

    if let Some(end_date) = payload.end_date {
        sqlx::query!(
            "UPDATE budgets SET end_date = $1 WHERE id = $2",
            end_date,
            id
        )
        .execute(&pool)
        .await?;
    }

    let budget = sqlx::query_as!(Budget, "SELECT * FROM budgets WHERE id = $1", id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(budget))
}

async fn delete_budget(
    AuthUser { user_id }: AuthUser,
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    sqlx::query!(
        "DELETE FROM budgets WHERE id = $1 AND user_id = $2",
        id,
        user_id
    )
    .execute(&pool)
    .await?;

    Ok(Json(()))
}

#[derive(Serialize)]
struct BudgetPerformance {
    budget: Budget,
    spent: f64,
    remaining: f64,
    percentage: f64,
}

async fn budget_performance(
    AuthUser { user_id }: AuthUser,
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<BudgetPerformance>, AppError> {
    let budget = sqlx::query_as!(
        Budget,
        "SELECT * FROM budgets WHERE id = $1 AND user_id = $2",
        id,
        user_id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;

    let end_date = budget
        .end_date
        .unwrap_or_else(|| chrono::Local::now().date_naive());

    let spent = sqlx::query_scalar!(
        "SELECT COALESCE(SUM(ABS(t.amount)), 0) as spent
         FROM transactions t
         JOIN accounts a ON t.account_id = a.id
         WHERE a.user_id = $1 
         AND t.category_id = $2
         AND t.date >= $3 
         AND t.date <= $4
         AND t.amount < 0",
        user_id,
        budget.category_id,
        budget.start_date,
        end_date
    )
    .fetch_one(&pool)
    .await?
    .unwrap_or(0.0);

    let remaining = budget.amount - spent;
    let percentage = if budget.amount > 0.0 {
        (spent / budget.amount) * 100.0
    } else {
        0.0
    };

    Ok(Json(BudgetPerformance {
        budget,
        spent,
        remaining,
        percentage,
    }))
}
