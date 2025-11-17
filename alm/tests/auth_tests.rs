#[cfg(test)]
mod tests {
    use super::super::common::TestContext;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_jwt_and_verify() {
        dotenv::dotenv().ok();
        std::env::set_var("JWT_SECRET", "test_secret_key");

        let user_id = Uuid::new_v4();
        let token = finance_backend::utils::auth::create_jwt(user_id).unwrap();

        let claims = finance_backend::utils::auth::verify_jwt(&token).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
    }

    #[tokio::test]
    async fn test_verify_invalid_jwt() {
        dotenv::dotenv().ok();
        std::env::set_var("JWT_SECRET", "test_secret_key");

        let result = finance_backend::utils::auth::verify_jwt("invalid_token");

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_register_user() {
        let ctx = TestContext::new().await;

        let new_email = format!("newuser_{}@example.com", Uuid::new_v4());
        let password = "NewPassword123!";
        let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();
        let user_id = Uuid::new_v4();

        let result = sqlx::query!(
            "INSERT INTO users (id, email, password_hash) VALUES ($1, $2, $3)",
            user_id,
            new_email,
            password_hash
        )
        .execute(&ctx.pool)
        .await;

        assert!(result.is_ok());

        let user = sqlx::query!("SELECT id, email FROM users WHERE email = $1", new_email)
            .fetch_one(&ctx.pool)
            .await
            .unwrap();

        assert_eq!(user.email, new_email);

        sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(&ctx.pool)
            .await
            .ok();

        ctx.cleanup().await;
    }

    #[tokio::test]
    async fn test_login_with_valid_credentials() {
        let ctx = TestContext::new().await;

        let user = sqlx::query!(
            "SELECT password_hash FROM users WHERE email = $1",
            ctx.test_user_email
        )
        .fetch_one(&ctx.pool)
        .await
        .unwrap();

        let valid = bcrypt::verify(&ctx.test_user_password, &user.password_hash).unwrap();
        assert!(valid);

        ctx.cleanup().await;
    }

    #[tokio::test]
    async fn test_login_with_invalid_credentials() {
        let ctx = TestContext::new().await;

        let user = sqlx::query!(
            "SELECT password_hash FROM users WHERE email = $1",
            ctx.test_user_email
        )
        .fetch_one(&ctx.pool)
        .await
        .unwrap();

        let valid = bcrypt::verify("WrongPassword", &user.password_hash).unwrap();
        assert!(!valid);

        ctx.cleanup().await;
    }
}
