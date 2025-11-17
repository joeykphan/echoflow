#[cfg(test)]
mod tests {
    use super::super::common::TestContext;
    use chrono::NaiveDate;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_calculate_net_worth() {
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
            "SELECT balance FROM accounts WHERE user_id = $1",
            ctx.test_user_id
        )
        .fetch_all(&ctx.pool)
        .await
        .unwrap();

        let total: f64 = accounts.iter().map(|a| a.balance).sum();
        assert_eq!(total, 6000.00);

        ctx.cleanup().await;
    }

    #[tokio::test]
    async fn test_spending_by_category() {
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

        let grocery_category = sqlx::query_scalar!(
            "SELECT id FROM categories WHERE name = 'Groceries' AND is_default = true LIMIT 1"
        )
        .fetch_one(&ctx.pool)
        .await
        .unwrap();

        let dining_category = sqlx::query_scalar!(
            "SELECT id FROM categories WHERE name = 'Dining Out' AND is_default = true LIMIT 1"
        )
        .fetch_one(&ctx.pool)
        .await
        .unwrap();

        let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();

        sqlx::query!(
            "INSERT INTO transactions (id, account_id, date, amount, description, category_id, pending) 
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
            Uuid::new_v4(),
            account_id,
            date,
            -100.00,
            "Grocery Store",
            grocery_category,
            false
        )
        .execute(&ctx.pool)
        .await
        .unwrap();

        sqlx::query!(
            "INSERT INTO transactions (id, account_id, date, amount, description, category_id, pending) 
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
            Uuid::new_v4(),
            account_id,
            date,
            -50.00,
            "Restaurant",
            dining_category,
            false
        )
        .execute(&ctx.pool)
        .await
        .unwrap();

        let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();

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
            ctx.test_user_id,
            start_date,
            end_date
        )
        .fetch_all(&ctx.pool)
        .await
        .unwrap();

        assert_eq!(spending.len(), 2);
        assert_eq!(spending[0].total.unwrap(), 100.00);
        assert_eq!(spending[1].total.unwrap(), 50.00);

        ctx.cleanup().await;
    }
}
