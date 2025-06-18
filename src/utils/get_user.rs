use crate::prelude::*;
use sqlx::Error;

pub async fn get_user(pool: &PgPool, api_key: String) -> Result<i32, Error> {
    let result: i32 = sqlx::query_scalar("SELECT owner_id FROM api_keys WHERE key = $1")
        .bind(&hash_api_key(&api_key))
        .fetch_one(pool)
        .await?;

    Ok(result)
}

pub async fn get_user_details(pool: &PgPool, email: String) -> Result<UserResponse, Error> {
    let result = sqlx::query_as::<_, UserResponse>(
        r#"
        SELECT 
            u.id, 
            u.email, 
            u.name, 
            u.password,
            COALESCE(array_agg(s.plan_name) FILTER (WHERE s.plan_name IS NOT NULL), '{}') AS plan_names
        FROM users u
        LEFT JOIN subscriptions s ON u.email = s.user_email
        WHERE u.email = $1
        GROUP BY u.id, u.email, u.name, u.password
        "#
    )
    .bind(&email)
    .fetch_one(pool)
    .await?;

    Ok(result)
}
