use crate::prelude::*;
use sqlx::Error;

pub async fn insert_user(pool: PgPool, user: User) -> Result<i32, Error> {
    let id: i32 = sqlx::query_scalar(
        "INSERT INTO users(email, name, company, password) VALUES($1, $2, $3, $4) RETURNING id",
    )
    .bind(&user.email)
    .bind(&user.name)
    .bind(&user.company)
    .bind(&user.password)
    .fetch_one(&pool)
    .await?;

    Ok(id)
}
