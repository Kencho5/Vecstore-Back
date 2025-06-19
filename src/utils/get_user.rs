use crate::prelude::*;
use sqlx::Error;

pub async fn get_user(pool: &PgPool, api_key: String, plan_name: String) -> Result<i32, Error> {
    let result: i32 = sqlx::query_scalar(
        "SELECT ak.owner_id 
         FROM api_keys ak
         INNER JOIN subscriptions s ON ak.owner_id = s.user_id
         WHERE ak.key = $1 
         AND s.status = 'active' 
         AND s.plan_name = $2",
    )
    .bind(&hash_api_key(&api_key))
    .bind(&plan_name)
    .fetch_one(pool)
    .await?;
    Ok(result)
}
