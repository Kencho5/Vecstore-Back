use crate::prelude::*;
use sqlx::Error;

pub async fn check_user(
    pool: &PgPool,
    email: String,
    password: Option<String>,
) -> Result<UserResponse, Error> {
    // Get user data in a single query
    let user = sqlx::query_as::<_, UserResponse>(
        "SELECT id, email, name, password FROM users WHERE email = $1",
    )
    .bind(&email)
    .fetch_optional(pool)
    .await?;

    match (user, password) {
        // User found and password provided (regular login)
        (Some(user), Some(password)) => {
            // Only verify if user has a password stored
            if let Some(stored_hash) = &user.password {
                // Verify password using bcrypt
                match bcrypt::verify(&password, stored_hash) {
                    true => Ok(user),
                    _ => Err(Error::RowNotFound), // Password didn't match
                }
            } else {
                // User exists but has no password (likely a Google user)
                Err(Error::RowNotFound)
            }
        }
        // User found but no password provided (Google login)
        (Some(user), None) => Ok(user),
        // User not found
        (None, _) => Err(Error::RowNotFound),
    }
}
