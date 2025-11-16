use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Account {
    pub id: Uuid,
    pub user_id: Uuid,
    pub plaid_account_id: Option<String>,
    pub plaid_item_id: Option<String>,
    pub account_name: String,
    pub account_type: String,
    pub balance: f64,
    pub currency: String,
    pub last_synced: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub account_id: Uuid,
    pub plaid_transaction_id: Option<String>,
    pub date: NaiveDate,
    pub amount: f64,
    pub description: String,
    pub category_id: Option<Uuid>,
    pub merchant_name: Option<String>,
    pub pending: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub category_type: String,
    pub color: String,
    pub icon: Option<String>,
    pub is_default: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Budget {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Uuid,
    pub amount: f64,
    pub period: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PlaidItem {
    pub id: Uuid,
    pub user_id: Uuid,
    pub plaid_access_token: String,
    pub plaid_item_id: String,
    pub institution_id: String,
    pub institution_name: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}
