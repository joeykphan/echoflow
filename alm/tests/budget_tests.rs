#[cfg(test)]
mod tests {
    use super::super::common::TestContext;
    use chrono::NaiveDate;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_budget() {
        let ctx = TestContext::new().await;

        let category_id = sqlx::query_scalar!(
            "SELECT id FROM categories WHERE name = 'Groceries' AND is_default = true LIMIT 1"
        )
        .fetch_one(&ctx.pool)
        .await
        .unwrap();

        let budget_id = Uuid::new_v4();
        let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

        let result = sqlx::query!(
            "INSERT INTO budgets (id, user_id, category_id, amount, period, start_date) 
             VALUES ($1, $2, $3, $4, $5, $6)",
            budget_id,
            ctx.test_user_id,
            category_id,
            500.00,
            "monthly",
            start_date
        )
        .execute(&ctx.pool)
        .await;

        assert!(result.is_ok());

        let budget = sqlx::query!(
            "SELECT amount, period FROM budgets WHERE id = $1",
            budget_id
        )
        .fetch_one(&ctx.pool)
        .await
        .unwrap();

        assert_eq!(budget.amount, 500.00);
        assert_eq!(budget.period, "monthly");

        ctx.cleanup().await;
    }

    #[tokio::test]
    async fn test_budget_performance_calculation() {
        let ctx = TestContext::new().await;

        let category_id = sqlx::query_scalar!(
            "SELECT id FROM categories WHERE name = 'Groceries' AND is_default = true LIMIT 1"
        )
        .fetch_one(&ctx.pool)
        .await
        .unwrap();

        let budget_id = Uuid::new_v4();
        let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();

        sqlx::query!(
            "INSERT INTO budgets (id, user_id, category_id, amount, period, start_date, end_date) 
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
            budget_id,
            ctx.test_user_id,
            category_id,
            500.00,
            "monthly",
            start_date,
            end_date
        )
        .execute(&ctx.pool)
        .await
        .unwrap();

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

        sqlx::query!(
            "INSERT INTO transactions (id, account_id, date, amount, description, category_id, pending) 
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
            Uuid::new_v4(),
            account_id,
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            -150.00,
            "Grocery Shopping",
            category_id,
            false
        )
        .execute(&ctx.pool)
        .await
        .unwrap();

        let spent = sqlx::query_scalar!(
            "SELECT COALESCE(SUM(ABS(t.amount)), 0) as spent
             FROM transactions t
             JOIN accounts a ON t.account_id = a.id
             WHERE a.user_id = $1 
             AND t.category_id = $2
             AND t.date >= $3 
             AND t.date <= $4
             AND t.amount < 0",
            ctx.test_user_id,
            category_id,
            start_date,
            end_date
        )
        .fetch_one(&ctx.pool)
        .await
        .unwrap()
        .unwrap_or(0.0);

        assert_eq!(spent, 150.00);

        ctx.cleanup().await;
    }
}
