use crate::prelude::*;
use sqlx::Error;

pub async fn check_user(
    pool: &PgPool,
    email: String,
    password: Option<String>,
) -> Result<UserResponse, Error> {
    let user = sqlx::query_as::<_, UserResponse>(
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
    .fetch_optional(pool)
    .await?;

    match (user, password) {
        (Some(user), Some(password)) => {
            if let Some(stored_hash) = &user.password {
                if bcrypt::verify(&password, stored_hash) {
                    Ok(user)
                } else {
                    Err(Error::RowNotFound)
                }
            } else {
                Err(Error::RowNotFound)
            }
        }
        (Some(user), None) => Ok(user),
        (None, _) => Err(Error::RowNotFound),
    }
}
