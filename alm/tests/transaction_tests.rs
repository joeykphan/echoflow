#[cfg(test)]
mod tests {
    use super::super::common::TestContext;
    use chrono::NaiveDate;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_transaction() {
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

        let transaction_id = Uuid::new_v4();
        let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();

        let result = sqlx::query!(
            "INSERT INTO transactions (id, account_id, date, amount, description, pending) 
             VALUES ($1, $2, $3, $4, $5, $6)",
            transaction_id,
            account_id,
            date,
            -50.00,
            "Grocery Store",
            false
        )
        .execute(&ctx.pool)
        .await;

        assert!(result.is_ok());

        let transaction = sqlx::query!(
            "SELECT description, amount FROM transactions WHERE id = $1",
            transaction_id
        )
        .fetch_one(&ctx.pool)
        .await
        .unwrap();

        assert_eq!(transaction.description, "Grocery Store");
        assert_eq!(transaction.amount, -50.00);

        ctx.cleanup().await;
    }

    #[tokio::test]
    async fn test_update_transaction_category() {
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

        let category_id = sqlx::query_scalar!(
            "SELECT id FROM categories WHERE name = 'Groceries' AND is_default = true LIMIT 1"
        )
        .fetch_one(&ctx.pool)
        .await
        .unwrap();

        let transaction_id = Uuid::new_v4();
        let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();

        sqlx::query!(
            "INSERT INTO transactions (id, account_id, date, amount, description, pending) 
             VALUES ($1, $2, $3, $4, $5, $6)",
            transaction_id,
            account_id,
            date,
            -50.00,
            "Grocery Store",
            false
        )
        .execute(&ctx.pool)
        .await
        .unwrap();

        sqlx::query!(
            "UPDATE transactions SET category_id = $1, updated_at = NOW() WHERE id = $2",
            category_id,
            transaction_id
        )
        .execute(&ctx.pool)
        .await
        .unwrap();

        let transaction = sqlx::query!(
            "SELECT category_id FROM transactions WHERE id = $1",
            transaction_id
        )
        .fetch_one(&ctx.pool)
        .await
        .unwrap();

        assert_eq!(transaction.category_id, Some(category_id));

        ctx.cleanup().await;
    }

    #[tokio::test]
    async fn test_filter_transactions_by_date() {
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

        let date1 = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 2, 15).unwrap();

        sqlx::query!(
            "INSERT INTO transactions (id, account_id, date, amount, description, pending) 
             VALUES ($1, $2, $3, $4, $5, $6)",
            Uuid::new_v4(),
            account_id,
            date1,
            -50.00,
            "January Transaction",
            false
        )
        .execute(&ctx.pool)
        .await
        .unwrap();

        sqlx::query!(
            "INSERT INTO transactions (id, account_id, date, amount, description, pending) 
             VALUES ($1, $2, $3, $4, $5, $6)",
            Uuid::new_v4(),
            account_id,
            date2,
            -75.00,
            "February Transaction",
            false
        )
        .execute(&ctx.pool)
        .await
        .unwrap();

        let start_date = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 2, 28).unwrap();

        let transactions = sqlx::query!(
            "SELECT description FROM transactions WHERE account_id = $1 AND date >= $2 AND date <= $3",
            account_id,
            start_date,
            end_date
        )
        .fetch_all(&ctx.pool)
        .await
        .unwrap();

        assert_eq!(transactions.len(), 1);
        assert_eq!(transactions[0].description, "February Transaction");

        ctx.cleanup().await;
    }
}
