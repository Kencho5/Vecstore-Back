use crate::prelude::*;
use sqlx::Error;

pub async fn insert_user(pool: PgPool, user: User) -> Result<(), Error> {
    let pwh = user
        .password
        .map(|p| bcrypt::hash(p).unwrap())
        .unwrap_or_default();

    sqlx::query("INSERT INTO users(email, name, password) VALUES($1, $2, $3)")
        .bind(&user.email)
        .bind(&user.name)
        .bind(&pwh)
        .execute(&pool)
        .await?;

    Ok(())
}
