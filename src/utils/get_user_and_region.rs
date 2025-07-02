use crate::prelude::*;

pub struct UserValidationResult {
    pub user_id: i32,
    pub region: String,
    pub credits_left: i32,
}

pub struct UserNsfwValidationResult {
    pub user_id: i32,
    pub credits_left: i32,
}

pub async fn validate_user_and_increment(
    pool: &PgPool,
    api_key: String,
    database: &str,
) -> Result<UserValidationResult, ApiError> {
    let result: Option<(i32, String, i32)> = sqlx::query_as(
        "WITH validated_user AS (
           SELECT ak.owner_id, d.db_type, d.region
           FROM api_keys ak
           JOIN databases d ON d.owner_id = ak.owner_id
           WHERE ak.key = $1 AND d.name = $2
         ),
         updated_credits AS (
           UPDATE users 
           SET credits = credits - 1 
           WHERE id = (SELECT owner_id FROM validated_user) AND credits > 0
           RETURNING id, credits
         ),
         updated_db AS (
           UPDATE databases 
           SET requests = requests + 1 
           WHERE name = $2 AND owner_id = (SELECT owner_id FROM validated_user)
           RETURNING owner_id, region
         )
         SELECT ud.owner_id, ud.region, uc.credits
         FROM updated_db ud
         JOIN updated_credits uc ON ud.owner_id = uc.id",
    )
    .bind(&hash_api_key(&api_key))
    .bind(database)
    .fetch_optional(pool)
    .await
    .map_err(|_| ApiError::DatabaseInsert)?;

    let (user_id, region, credits_left) = match result {
        Some((id, reg, credits)) => (id, reg, credits),
        None => {
            let api_key_check: Option<(i32,)> =
                sqlx::query_as("SELECT owner_id FROM api_keys WHERE key = $1")
                    .bind(&hash_api_key(&api_key))
                    .fetch_optional(pool)
                    .await
                    .map_err(|_| ApiError::InvalidApiKey)?;

            match api_key_check {
                Some(_) => {
                    let db_check: Option<(i32,)> = sqlx::query_as(
                        "SELECT ak.owner_id 
                         FROM api_keys ak
                         JOIN databases d ON d.owner_id = ak.owner_id
                         WHERE ak.key = $1 AND d.name = $2",
                    )
                    .bind(&hash_api_key(&api_key))
                    .bind(database)
                    .fetch_optional(pool)
                    .await
                    .map_err(|_| ApiError::DatabaseInsert)?;

                    match db_check {
                        Some(_) => return Err(ApiError::RequestLimitExceeded),
                        None => return Err(ApiError::DatabaseNotFound),
                    }
                }
                None => return Err(ApiError::InvalidApiKey),
            }
        }
    };

    Ok(UserValidationResult {
        user_id,
        region,
        credits_left,
    })
}

pub async fn validate_nsfw_request(
    pool: &PgPool,
    api_key: String,
) -> Result<UserNsfwValidationResult, ApiError> {
    let result: Option<(i32, i32)> = sqlx::query_as(
        "WITH validated_user AS (
           SELECT owner_id FROM api_keys WHERE key = $1
         ),
         updated_credits AS (
           UPDATE users 
           SET credits = credits - 1 
           WHERE id = (SELECT owner_id FROM validated_user) AND credits > 0
           RETURNING id, credits
         )
         SELECT id, credits FROM updated_credits",
    )
    .bind(&hash_api_key(&api_key))
    .fetch_optional(pool)
    .await
    .map_err(|_| ApiError::DatabaseInsert)?;

    let (user_id, credits_left) = match result {
        Some((id, credits)) => (id, credits),
        None => {
            let validation_check: Option<(i32,)> = sqlx::query_as(
                "SELECT owner_id 
                 FROM api_keys
                 WHERE key = $1",
            )
            .bind(&hash_api_key(&api_key))
            .fetch_optional(pool)
            .await
            .map_err(|_| ApiError::InvalidApiKey)?;

            match validation_check {
                Some(_) => return Err(ApiError::RequestLimitExceeded),
                None => return Err(ApiError::InvalidApiKey),
            }
        }
    };

    Ok(UserNsfwValidationResult {
        user_id,
        credits_left,
    })
}
