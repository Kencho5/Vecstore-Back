use crate::{prelude::*, structs::dashboard_struct::*};
use sqlx::Error;

pub async fn increment_req(
    pool: &PgPool,
    database: &String,
    user_id: i32,
) -> Result<String, Error> {
    let row = sqlx::query_as::<_, DatabaseInfo>(
        "UPDATE databases
     SET requests = requests + 1
     WHERE name = $1 AND owner_id = $2
     RETURNING region, db_type",
    )
    .bind(&database)
    .bind(&user_id)
    .fetch_one(pool)
    .await?;

    Ok(format!("{}-{}", row.region, row.db_type))
}
