#[cfg(test)]
mod tests {
    use super::super::common::TestContext;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_account() {
        let ctx = TestContext::new().await;

        let account_id = Uuid::new_v4();
        let result = sqlx::query!(
            "INSERT INTO accounts (id, user_id, account_name, account_type, balance, currency) 
             VALUES ($1, $2, $3, $4, $5, $6)",
            account_id,
            ctx.test_user_id,
            "Test Checking",
            "checking",
            1000.00,
            "USD"
        )
        .execute(&ctx.pool)
        .await;

        assert!(result.is_ok());

        let account = sqlx::query!(
            "SELECT account_name, balance FROM accounts WHERE id = $1",
            account_id
        )
        .fetch_one(&ctx.pool)
        .await
        .unwrap();

        assert_eq!(account.account_name, "Test Checking");
        assert_eq!(account.balance, 1000.00);

        ctx.cleanup().await;
    }

    #[tokio::test]
    async fn test_list_user_accounts() {
        let ctx = TestContext::new().await;

        let account1_id = Uuid::new_v4();
        let account2_id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO accounts (id, user_id, account_name, account_type, balance, currency) 
             VALUES ($1, $2, $3, $4, $5, $6)",
            account1_id,
            ctx.test_user_id,
            "Checking",
            "checking",
            1000.00,
            "USD"
        )
        .execute(&ctx.pool)
        .await
        .unwrap();

        sqlx::query!(
            "INSERT INTO accounts (id, user_id, account_name, account_type, balance, currency) 
             VALUES ($1, $2, $3, $4, $5, $6)",
            account2_id,
            ctx.test_user_id,
            "Savings",
            "savings",
            5000.00,
            "USD"
        )
        .execute(&ctx.pool)
        .await
        .unwrap();

        let accounts = sqlx::query!(
            "SELECT id FROM accounts WHERE user_id = $1",
            ctx.test_user_id
        )
        .fetch_all(&ctx.pool)
        .await
        .unwrap();

        assert_eq!(accounts.len(), 2);

        ctx.cleanup().await;
    }

    #[tokio::test]
    async fn test_delete_account() {
        let ctx = TestContext::new().await;

        let account_id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO accounts (id, user_id, account_name, account_type, balance, currency) 
             VALUES ($1, $2, $3, $4, $5, $6)",
            account_id,
            ctx.test_user_id,
            "Test Account",
            "checking",
            1000.00,
            "USD"
        )
        .execute(&ctx.pool)
        .await
        .unwrap();

        let result = sqlx::query!(
            "DELETE FROM accounts WHERE id = $1 AND user_id = $2",
            account_id,
            ctx.test_user_id
        )
        .execute(&ctx.pool)
        .await;

        assert!(result.is_ok());

        let account = sqlx::query!("SELECT id FROM accounts WHERE id = $1", account_id)
            .fetch_optional(&ctx.pool)
            .await
            .unwrap();

        assert!(account.is_none());

        ctx.cleanup().await;
    }
}
