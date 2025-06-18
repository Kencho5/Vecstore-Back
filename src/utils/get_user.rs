use crate::prelude::*;
use sqlx::Error;

pub async fn get_user(pool: &PgPool, api_key: String) -> Result<i32, Error> {
    let result: i32 = sqlx::query_scalar("SELECT owner_id FROM api_keys WHERE key = $1")
        .bind(&hash_api_key(&api_key))
        .fetch_one(pool)
        .await?;

    Ok(result)
}
