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
        WITH user_validation AS (
            SELECT 
                ak.owner_id,
                ak.key IS NOT NULL as key_exists,
                s.status = 'active' as subscription_active,
                s.usage_reset_date < (s.next_billing_date - INTERVAL '1 month') as should_reset,
                d.total_requests,
                s.req_limit,
                db.region
            FROM api_keys ak
            FULL OUTER JOIN subscriptions s ON ak.owner_id = s.user_id AND s.plan_name = $2
            FULL OUTER JOIN (
                SELECT owner_id, SUM(requests) AS total_requests
                FROM databases
                GROUP BY owner_id
            ) d ON ak.owner_id = d.owner_id
            JOIN databases db ON db.name = $3 AND db.owner_id = ak.owner_id
            WHERE ak.key = $1
        )
        SELECT 
            owner_id,
            key_exists,
            subscription_active,
            CASE 
                WHEN should_reset THEN 0 < req_limit
                ELSE COALESCE(total_requests, 0) < req_limit
            END as within_limits,
            should_reset,
            region
        FROM user_validation
        ",
    )
    .bind(&hash_api_key(api_key))
    .bind(plan_name)
    .bind(database)
    .fetch_optional(pool)
    .await
    .map_err(|_| InsertError::InvalidSubscription)?;

    match result {
        None => Err(InsertError::InvalidApiKey),
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
