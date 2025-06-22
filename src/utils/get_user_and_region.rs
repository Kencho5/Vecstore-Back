use crate::prelude::*;
use crate::structs::insert_struct::*;

#[derive(Debug, Clone)]
pub struct UserValidationResult {
    pub user_id: i32,
    pub region: String,
}

pub async fn validate_user_and_increment(
    pool: &PgPool,
    api_key: String,
    database: String,
    plan_name: String,
) -> Result<UserValidationResult, InsertError> {
    // First validate the user and check limits
    let user_id = validate_user_limits(pool, &api_key, &plan_name).await?;

    // Then atomically increment requests and get region
    let region: String = sqlx::query_scalar(
        "UPDATE databases 
         SET requests = requests + 1 
         WHERE name = $1 AND owner_id = $2
         RETURNING region",
    )
    .bind(&database)
    .bind(&user_id)
    .fetch_one(pool)
    .await
    .map_err(|_| InsertError::DatabaseNotFound)?;

    Ok(UserValidationResult { user_id, region })
}

//pub async fn validate_user_get_region(
//    pool: &PgPool,
//    api_key: String,
//    database: String,
//    plan_name: String,
//) -> Result<UserValidationResult, InsertError> {
//    let user_id = validate_user_limits(pool, &api_key, &plan_name).await?;
//
//    let region: String = sqlx::query_scalar(
//        "SELECT region FROM databases WHERE name = $1 AND owner_id = $2"
//    )
//    .bind(&database)
//    .bind(&user_id)
//    .fetch_one(pool)
//    .await
//    .map_err(|_| InsertError::DatabaseNotFound)?;
//
//    Ok(UserValidationResult { user_id, region })
//}

async fn validate_user_limits(
    pool: &PgPool,
    api_key: &str,
    plan_name: &str,
) -> Result<i32, InsertError> {
    let result: Option<(i32, bool, bool, bool)> = sqlx::query_as(
        "
        SELECT 
            ak.owner_id,
            ak.key IS NOT NULL as key_exists,
            s.status = 'active' as subscription_active,
            d.total_requests < s.req_limit as within_limits
        FROM api_keys ak
        FULL OUTER JOIN subscriptions s ON ak.owner_id = s.user_id AND s.plan_name = $2
        FULL OUTER JOIN (
          SELECT owner_id, SUM(requests) AS total_requests
          FROM databases
          GROUP BY owner_id
        ) d ON ak.owner_id = d.owner_id
        WHERE ak.key = $1
        ",
    )
    .bind(&hash_api_key(api_key))
    .bind(plan_name)
    .fetch_optional(pool)
    .await
    .map_err(|_| InsertError::InvalidSubscription)?;

    match result {
        None => Err(InsertError::InvalidApiKey),
        Some((user_id, key_exists, subscription_active, within_limits)) => {
            if !key_exists {
                Err(InsertError::InvalidApiKey)
            } else if !subscription_active {
                Err(InsertError::InvalidSubscription)
            } else if !within_limits {
                Err(InsertError::RequestLimitExceeded)
            } else {
                Ok(user_id)
            }
        }
    }
}

