use crate::prelude::*;
use sqlx::Error;

pub async fn check_user(pool: PgPool, user: User) -> Result<(), Error> {
    let pwh = user
        .password
        .map(|p| bcrypt::hash(p).unwrap())
        .unwrap_or_default();

    sqlx::query("SELECT email FROM users WHERE email = $1 AND password = $2 ")
        .bind(&user.email)
        .bind(&pwh)
        .fetch_one(&pool)
        .await?;

    Ok(())
}
