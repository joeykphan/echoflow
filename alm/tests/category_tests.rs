#[cfg(test)]
mod tests {
    use super::super::common::TestContext;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_custom_category() {
        let ctx = TestContext::new().await;

        let category_id = Uuid::new_v4();
        let result = sqlx::query!(
            "INSERT INTO categories (id, user_id, name, category_type, color, icon, is_default) 
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
            category_id,
            ctx.test_user_id,
            "Custom Category",
            "expense",
            "#ff0000",
            "🎯",
            false
        )
        .execute(&ctx.pool)
        .await;

        assert!(result.is_ok());

        let category = sqlx::query!(
            "SELECT name, color FROM categories WHERE id = $1",
            category_id
        )
        .fetch_one(&ctx.pool)
        .await
        .unwrap();

        assert_eq!(category.name, "Custom Category");
        assert_eq!(category.color, "#ff0000");

        ctx.cleanup().await;
    }

    #[tokio::test]
    async fn test_list_default_categories() {
        let ctx = TestContext::new().await;

        let categories =
            sqlx::query!("SELECT name FROM categories WHERE is_default = true ORDER BY name")
                .fetch_all(&ctx.pool)
                .await
                .unwrap();

        assert!(categories.len() >= 10);
        assert!(categories.iter().any(|c| c.name == "Groceries"));
        assert!(categories.iter().any(|c| c.name == "Income"));

        ctx.cleanup().await;
    }

    #[tokio::test]
    async fn test_update_category() {
        let ctx = TestContext::new().await;

        let category_id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO categories (id, user_id, name, category_type, color, icon, is_default) 
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
            category_id,
            ctx.test_user_id,
            "Original Name",
            "expense",
            "#ff0000",
            "🎯",
            false
        )
        .execute(&ctx.pool)
        .await
        .unwrap();

        sqlx::query!(
            "UPDATE categories SET name = $1, color = $2 WHERE id = $3",
            "Updated Name",
            "#00ff00",
            category_id
        )
        .execute(&ctx.pool)
        .await
        .unwrap();

        let category = sqlx::query!(
            "SELECT name, color FROM categories WHERE id = $1",
            category_id
        )
        .fetch_one(&ctx.pool)
        .await
        .unwrap();

        assert_eq!(category.name, "Updated Name");
        assert_eq!(category.color, "#00ff00");

        ctx.cleanup().await;
    }
}
