use sqlx::{postgres::PgPoolOptions, PgPool};
use uuid::Uuid;

pub struct TestContext {
    pub pool: PgPool,
    pub test_user_id: Uuid,
    pub test_user_email: String,
    pub test_user_password: String,
}

impl TestContext {
    pub async fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgresql://postgres:password@localhost:5432/financeapp_test".to_string()
        });

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to connect to test database");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        let test_user_email = format!("test_{}@example.com", Uuid::new_v4());
        let test_user_password = "TestPassword123!";
        let password_hash = bcrypt::hash(test_user_password, bcrypt::DEFAULT_COST).unwrap();
        let test_user_id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO users (id, email, password_hash) VALUES ($1, $2, $3)",
            test_user_id,
            test_user_email,
            password_hash
        )
        .execute(&pool)
        .await
        .expect("Failed to create test user");

        Self {
            pool,
            test_user_id,
            test_user_email,
            test_user_password: test_user_password.to_string(),
        }
    }

    pub async fn cleanup(&self) {
        sqlx::query!("DELETE FROM transactions WHERE account_id IN (SELECT id FROM accounts WHERE user_id = $1)", self.test_user_id)
            .execute(&self.pool)
            .await
            .ok();

        sqlx::query!("DELETE FROM budgets WHERE user_id = $1", self.test_user_id)
            .execute(&self.pool)
            .await
            .ok();

        sqlx::query!("DELETE FROM accounts WHERE user_id = $1", self.test_user_id)
            .execute(&self.pool)
            .await
            .ok();

        sqlx::query!(
            "DELETE FROM categories WHERE user_id = $1",
            self.test_user_id
        )
        .execute(&self.pool)
        .await
        .ok();

        sqlx::query!(
            "DELETE FROM plaid_items WHERE user_id = $1",
            self.test_user_id
        )
        .execute(&self.pool)
        .await
        .ok();

        sqlx::query!("DELETE FROM users WHERE id = $1", self.test_user_id)
            .execute(&self.pool)
            .await
            .ok();
    }
}
