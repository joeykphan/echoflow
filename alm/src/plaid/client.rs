use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct PlaidClient {
    client: reqwest::Client,
    client_id: String,
    secret: String,
    env: String,
}

impl PlaidClient {
    pub fn new() -> Self {
        let client_id = std::env::var("PLAID_CLIENT_ID").expect("PLAID_CLIENT_ID must be set");
        let secret = std::env::var("PLAID_SECRET").expect("PLAID_SECRET must be set");
        let env = std::env::var("PLAID_ENV").unwrap_or_else(|_| "sandbox".to_string());

        Self {
            client: reqwest::Client::new(),
            client_id,
            secret,
            env,
        }
    }

    fn base_url(&self) -> &str {
        match self.env.as_str() {
            "production" => "https://production.plaid.com",
            "development" => "https://development.plaid.com",
            _ => "https://sandbox.plaid.com",
        }
    }

    pub async fn create_link_token(
        &self,
        user_id: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        #[derive(Serialize)]
        struct LinkTokenRequest {
            client_id: String,
            secret: String,
            user: User,
            client_name: String,
            products: Vec<String>,
            country_codes: Vec<String>,
            language: String,
        }

        #[derive(Serialize)]
        struct User {
            client_user_id: String,
        }

        #[derive(Deserialize)]
        struct LinkTokenResponse {
            link_token: String,
        }

        let request = LinkTokenRequest {
            client_id: self.client_id.clone(),
            secret: self.secret.clone(),
            user: User {
                client_user_id: user_id,
            },
            client_name: "Finance Budget App".to_string(),
            products: vec!["transactions".to_string()],
            country_codes: vec!["US".to_string()],
            language: "en".to_string(),
        };

        let response = self
            .client
            .post(format!("{}/link/token/create", self.base_url()))
            .json(&request)
            .send()
            .await?
            .json::<LinkTokenResponse>()
            .await?;

        Ok(response.link_token)
    }

    pub async fn exchange_public_token(
        &self,
        public_token: &str,
    ) -> Result<(String, String), Box<dyn std::error::Error>> {
        #[derive(Serialize)]
        struct ExchangeRequest {
            client_id: String,
            secret: String,
            public_token: String,
        }

        #[derive(Deserialize)]
        struct ExchangeResponse {
            access_token: String,
            item_id: String,
        }

        let request = ExchangeRequest {
            client_id: self.client_id.clone(),
            secret: self.secret.clone(),
            public_token: public_token.to_string(),
        };

        let response = self
            .client
            .post(format!("{}/item/public_token/exchange", self.base_url()))
            .json(&request)
            .send()
            .await?
            .json::<ExchangeResponse>()
            .await?;

        Ok((response.access_token, response.item_id))
    }

    pub async fn get_item(
        &self,
        access_token: &str,
    ) -> Result<ItemInfo, Box<dyn std::error::Error>> {
        #[derive(Serialize)]
        struct ItemRequest {
            client_id: String,
            secret: String,
            access_token: String,
        }

        #[derive(Deserialize)]
        struct ItemResponse {
            item: Item,
        }

        #[derive(Deserialize)]
        struct Item {
            institution_id: Option<String>,
        }

        let request = ItemRequest {
            client_id: self.client_id.clone(),
            secret: self.secret.clone(),
            access_token: access_token.to_string(),
        };

        let response = self
            .client
            .post(format!("{}/item/get", self.base_url()))
            .json(&request)
            .send()
            .await?
            .json::<ItemResponse>()
            .await?;

        let institution_id = response.item.institution_id.unwrap_or_default();

        Ok(ItemInfo {
            institution_id: institution_id.clone(),
            institution_name: institution_id,
        })
    }

    pub async fn get_accounts(
        &self,
        access_token: &str,
    ) -> Result<Vec<AccountInfo>, Box<dyn std::error::Error>> {
        #[derive(Serialize)]
        struct AccountsRequest {
            client_id: String,
            secret: String,
            access_token: String,
        }

        #[derive(Deserialize)]
        struct AccountsResponse {
            accounts: Vec<Account>,
        }

        #[derive(Deserialize)]
        struct Account {
            account_id: String,
            name: String,
            #[serde(rename = "type")]
            account_type: String,
            balances: Balances,
        }

        #[derive(Deserialize)]
        struct Balances {
            current: Option<f64>,
        }

        let request = AccountsRequest {
            client_id: self.client_id.clone(),
            secret: self.secret.clone(),
            access_token: access_token.to_string(),
        };

        let response = self
            .client
            .post(format!("{}/accounts/get", self.base_url()))
            .json(&request)
            .send()
            .await?
            .json::<AccountsResponse>()
            .await?;

        Ok(response
            .accounts
            .into_iter()
            .map(|a| AccountInfo {
                account_id: a.account_id,
                name: a.name,
                account_type: a.account_type,
                balance: a.balances.current.unwrap_or(0.0),
            })
            .collect())
    }

    pub async fn get_transactions(
        &self,
        access_token: &str,
        days: i64,
    ) -> Result<Vec<TransactionInfo>, Box<dyn std::error::Error>> {
        #[derive(Serialize)]
        struct TransactionsRequest {
            client_id: String,
            secret: String,
            access_token: String,
            start_date: String,
            end_date: String,
        }

        #[derive(Deserialize)]
        struct TransactionsResponse {
            transactions: Vec<Transaction>,
        }

        #[derive(Deserialize)]
        struct Transaction {
            transaction_id: String,
            account_id: String,
            amount: f64,
            date: String,
            name: String,
            merchant_name: Option<String>,
            pending: bool,
        }

        let end_date = Utc::now().date_naive();
        let start_date = end_date - Duration::days(days);

        let request = TransactionsRequest {
            client_id: self.client_id.clone(),
            secret: self.secret.clone(),
            access_token: access_token.to_string(),
            start_date: start_date.to_string(),
            end_date: end_date.to_string(),
        };

        let response = self
            .client
            .post(format!("{}/transactions/get", self.base_url()))
            .json(&request)
            .send()
            .await?
            .json::<TransactionsResponse>()
            .await?;

        Ok(response
            .transactions
            .into_iter()
            .map(|t| TransactionInfo {
                transaction_id: t.transaction_id,
                account_id: t.account_id,
                amount: t.amount,
                date: chrono::NaiveDate::parse_from_str(&t.date, "%Y-%m-%d").unwrap(),
                name: t.name,
                merchant_name: t.merchant_name,
                pending: t.pending,
            })
            .collect())
    }
}

pub struct ItemInfo {
    pub institution_id: String,
    pub institution_name: String,
}

pub struct AccountInfo {
    pub account_id: String,
    pub name: String,
    pub account_type: String,
    pub balance: f64,
}

pub struct TransactionInfo {
    pub transaction_id: String,
    pub account_id: String,
    pub amount: f64,
    pub date: chrono::NaiveDate,
    pub name: String,
    pub merchant_name: Option<String>,
    pub pending: bool,
}
