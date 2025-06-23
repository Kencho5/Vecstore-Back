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
    let result = validate_and_process_request(pool, &api_key, &database, &plan_name).await?;

    Ok(UserValidationResult {
        user_id: result.0,
        region: result.1,
    })
}

async fn validate_and_process_request(
    pool: &PgPool,
    api_key: &str,
    database: &str,
    plan_name: &str,
) -> Result<(i32, String), InsertError> {
    let result: Option<(i32, bool, bool, bool, bool, String)> = sqlx::query_as(
        "
        SELECT 
            ak.owner_id,
            ak.key IS NOT NULL as key_exists,
            s.status = 'active' as subscription_active,
            CASE 
                WHEN s.usage_reset_date < (s.next_billing_date - INTERVAL '1 month') THEN 0 < s.req_limit
                ELSE COALESCE((
                    SELECT SUM(requests) 
                    FROM databases 
                    WHERE owner_id = ak.owner_id AND db_type = s.db_type
                ), 0) < s.req_limit
            END as within_limits,
            s.usage_reset_date < (s.next_billing_date - INTERVAL '1 month') as should_reset,
            db.region
        FROM api_keys ak
        JOIN databases db ON db.name = $3 AND db.owner_id = ak.owner_id
        JOIN subscriptions s ON s.user_id = ak.owner_id AND s.plan_name = $2 AND s.status = 'active'
        WHERE ak.key = $1
        ",
    )
    .bind(&hash_api_key(api_key))
    .bind(plan_name)
    .bind(database)
    .fetch_optional(pool)
    .await
    .map_err(|_| InsertError::InvalidSubscription)?;

    match result {
        None => Err(InsertError::InvalidSubscription),
        Some((user_id, key_exists, subscription_active, within_limits, should_reset, region)) => {
            if !key_exists {
                Err(InsertError::InvalidApiKey)
            } else if !subscription_active {
                Err(InsertError::InvalidSubscription)
            } else if !within_limits {
                Err(InsertError::RequestLimitExceeded)
            } else {
                // Reset usage if needed, then increment
                if should_reset {
                    // Reset database usage counters
                    sqlx::query(
                        "UPDATE databases SET requests = 1 WHERE owner_id = $1 AND db_type IN (SELECT db_type FROM subscriptions WHERE user_id = $1 AND plan_name = $2 AND status = 'active')"
                    )
                    .bind(user_id)
                    .bind(plan_name)
                    .execute(pool)
                    .await
                    .map_err(|_| InsertError::InvalidSubscription)?;

                    // Update usage_reset_date to current date
                    sqlx::query(
                        "UPDATE subscriptions SET usage_reset_date = CURRENT_DATE WHERE user_id = $1 AND plan_name = $2 AND status = 'active'"
                    )
                    .bind(user_id)
                    .bind(plan_name)
                    .execute(pool)
                    .await
                    .map_err(|_| InsertError::InvalidSubscription)?;
                } else {
                    sqlx::query(
                        "UPDATE databases SET requests = requests + 1 WHERE name = $1 AND owner_id = $2"
                    )
                    .bind(database)
                    .bind(user_id)
                    .execute(pool)
                    .await
                    .map_err(|_| InsertError::DatabaseNotFound)?;
                }

                Ok((user_id, region))
            }
        }
    }
}
