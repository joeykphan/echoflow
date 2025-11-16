use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db::DbPool,
    utils::{auth::AuthUser, AppError},
};

pub fn routes(pool: DbPool) -> Router {
    Router::new()
        .route("/net-worth", get(net_worth))
        .route("/spending-by-category", get(spending_by_category))
        .route("/income-over-time", get(income_over_time))
        .route("/spending-over-time", get(spending_over_time))
        .with_state(pool)
}

#[derive(Serialize)]
struct NetWorthResponse {
    total: f64,
    accounts: Vec<AccountBalance>,
}

#[derive(Serialize)]
struct AccountBalance {
    account_id: Uuid,
    account_name: String,
    balance: f64,
}

async fn net_worth(
    AuthUser { user_id }: AuthUser,
    State(pool): State<DbPool>,
) -> Result<Json<NetWorthResponse>, AppError> {
    let accounts = sqlx::query!(
        "SELECT id, account_name, balance FROM accounts WHERE user_id = $1",
        user_id
    )
    .fetch_all(&pool)
    .await?;

    let total: f64 = accounts.iter().map(|a| a.balance).sum();

    let account_balances = accounts
        .into_iter()
        .map(|a| AccountBalance {
            account_id: a.id,
            account_name: a.account_name,
            balance: a.balance,
        })
        .collect();

    Ok(Json(NetWorthResponse {
        total,
        accounts: account_balances,
    }))
}

#[derive(Serialize)]
struct CategorySpending {
    category_id: Option<Uuid>,
    category_name: Option<String>,
    total: f64,
    percentage: f64,
}

#[derive(Deserialize)]
struct DateRangeQuery {
    start_date: NaiveDate,
    end_date: NaiveDate,
}

async fn spending_by_category(
    AuthUser { user_id }: AuthUser,
    State(pool): State<DbPool>,
    Query(query): Query<DateRangeQuery>,
) -> Result<Json<Vec<CategorySpending>>, AppError> {
    let total_spending = sqlx::query_scalar!(
        "SELECT COALESCE(SUM(ABS(t.amount)), 0) as total
         FROM transactions t
         JOIN accounts a ON t.account_id = a.id
         WHERE a.user_id = $1
         AND t.date >= $2
         AND t.date <= $3
         AND t.amount < 0",
        user_id,
        query.start_date,
        query.end_date
    )
    .fetch_one(&pool)
    .await?
    .unwrap_or(0.0);

    let spending = sqlx::query!(
        "SELECT 
            t.category_id,
            c.name as category_name,
            SUM(ABS(t.amount)) as total
         FROM transactions t
         JOIN accounts a ON t.account_id = a.id
         LEFT JOIN categories c ON t.category_id = c.id
         WHERE a.user_id = $1
         AND t.date >= $2
         AND t.date <= $3
         AND t.amount < 0
         GROUP BY t.category_id, c.name
         ORDER BY total DESC",
        user_id,
        query.start_date,
        query.end_date
    )
    .fetch_all(&pool)
    .await?;

    let result = spending
        .into_iter()
        .map(|s| {
            let total = s.total.unwrap_or(0.0);
            CategorySpending {
                category_id: s.category_id,
                category_name: s.category_name,
                total,
                percentage: if total_spending > 0.0 {
                    (total / total_spending) * 100.0
                } else {
                    0.0
                },
            }
        })
        .collect();

    Ok(Json(result))
}

#[derive(Serialize)]
struct TimeSeriesData {
    date: NaiveDate,
    amount: f64,
}

async fn income_over_time(
    AuthUser { user_id }: AuthUser,
    State(pool): State<DbPool>,
    Query(query): Query<DateRangeQuery>,
) -> Result<Json<Vec<TimeSeriesData>>, AppError> {
    let data = sqlx::query!(
        "SELECT 
            t.date,
            SUM(t.amount) as amount
         FROM transactions t
         JOIN accounts a ON t.account_id = a.id
         WHERE a.user_id = $1
         AND t.date >= $2
         AND t.date <= $3
         AND t.amount > 0
         GROUP BY t.date
         ORDER BY t.date",
        user_id,
        query.start_date,
        query.end_date
    )
    .fetch_all(&pool)
    .await?;

    let result = data
        .into_iter()
        .map(|d| TimeSeriesData {
            date: d.date,
            amount: d.amount.unwrap_or(0.0),
        })
        .collect();

    Ok(Json(result))
}

async fn spending_over_time(
    AuthUser { user_id }: AuthUser,
    State(pool): State<DbPool>,
    Query(query): Query<DateRangeQuery>,
) -> Result<Json<Vec<TimeSeriesData>>, AppError> {
    let data = sqlx::query!(
        "SELECT 
            t.date,
            SUM(ABS(t.amount)) as amount
         FROM transactions t
         JOIN accounts a ON t.account_id = a.id
         WHERE a.user_id = $1
         AND t.date >= $2
         AND t.date <= $3
         AND t.amount < 0
         GROUP BY t.date
         ORDER BY t.date",
        user_id,
        query.start_date,
        query.end_date
    )
    .fetch_all(&pool)
    .await?;

    let result = data
        .into_iter()
        .map(|d| TimeSeriesData {
            date: d.date,
            amount: d.amount.unwrap_or(0.0),
        })
        .collect();

    Ok(Json(result))
}
