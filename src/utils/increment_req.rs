use crate::prelude::*;
use sqlx::Error;

pub async fn increment_req(
    pool: &PgPool,
    database: &String,
    user_id: i32,
) -> Result<String, Error> {
    let region: String = sqlx::query_scalar(
        "UPDATE databases
     SET requests = requests + 1
     WHERE name = $1 AND owner_id = $2
     RETURNING region",
    )
    .bind(&database)
    .bind(&user_id)
    .fetch_one(pool)
    .await?;

    Ok(region)
}
