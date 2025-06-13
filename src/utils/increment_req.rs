use crate::prelude::*;
use sqlx::Error;

pub async fn increment_req(pool: &PgPool, database: String, user_id: i32) -> Result<(), Error> {
    sqlx::query("UPDATE databases SET requests = requests + 1 WHERE name = $1 AND owner_id = $2 ")
        .bind(&database)
        .bind(&user_id)
        .execute(pool)
        .await?;

    Ok(())
}
