use crate::prelude::*;
use crate::structs::insert_struct::*;

pub async fn get_user_key(
    pool: &PgPool,
    api_key: String,
    plan_name: String,
) -> Result<i32, InsertImageError> {
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
    .bind(&hash_api_key(&api_key))
    .bind(&plan_name)
    .fetch_optional(pool)
    .await
    .map_err(|_| InsertImageError::DatabaseConnection)?;

    match result {
        None => Err(InsertImageError::InvalidApiKey),
        Some((owner_id, key_exists, subscription_active, within_limits)) => {
            if !key_exists {
                Err(InsertImageError::InvalidApiKey)
            } else if !subscription_active {
                Err(InsertImageError::InvalidSubscription)
            } else if !within_limits {
                Err(InsertImageError::RequestLimitExceeded)
            } else {
                Ok(owner_id)
            }
        }
    }
}
