use crate::prelude::*;
use sqlx::Error;

pub async fn check_user(
    pool: &PgPool,
    email: String,
    password: Option<String>,
) -> Result<UserResponse, Error> {
    let user = sqlx::query_as::<_, UserResponse>(
        "SELECT id, email, name, password FROM users WHERE email = $1",
    )
    .bind(&email)
    .fetch_optional(pool)
    .await?;

    match (user, password) {
        (Some(user), Some(password)) => {
            if let Some(stored_hash) = &user.password {
                match bcrypt::verify(&password, stored_hash) {
                    true => Ok(user),
                    _ => Err(Error::RowNotFound), // Password didn't match
                }
            } else {
                Err(Error::RowNotFound)
            }
        }
        // User found but no password provided (Google login)
        (Some(user), None) => Ok(user),
        // User not found
        (None, _) => Err(Error::RowNotFound),
    }
}
