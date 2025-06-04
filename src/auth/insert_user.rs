use crate::prelude::*;
use sqlx::Error;

pub async fn insert_user(pool: PgPool, user: User) -> Result<i32, Error> {
    let pwh = user
        .password
        .map(|p| bcrypt::hash(p).unwrap())
        .unwrap_or_default();

    let id: i32 = sqlx::query_scalar(
        "INSERT INTO users(email, name, password) VALUES($1, $2, $3) RETURNING id",
    )
    .bind(&user.email)
    .bind(&user.name)
    .bind(&pwh)
    .fetch_one(&pool)
    .await?;

    Ok(id)
}
