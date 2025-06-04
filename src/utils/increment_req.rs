use crate::prelude::*;
use sqlx::Error;

pub async fn increment_req(pool: &PgPool, database: String) -> Result<(), Error> {
    sqlx::query("UPDATE databases SET requests = requests + 1 WHERE name = $1 ")
        .bind(&database)
        .execute(pool)
        .await?;

    Ok(())
}
