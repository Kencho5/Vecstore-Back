use crate::prelude::*;

pub async fn validate_user_only(
    pool: &PgPool,
    api_key: String,
    database: &str,
) -> Result<UserCacheResult, ApiError> {
    let result: Option<(i32, String, i32, String)> = sqlx::query_as(
        "SELECT ak.owner_id, d.region, u.credits, d.db_type
         FROM api_keys ak
         JOIN databases d ON d.owner_id = ak.owner_id
         JOIN users u ON u.id = ak.owner_id
         WHERE ak.key = $1 AND d.name = $2",
    )
    .bind(&hash_api_key(&api_key))
    .bind(database)
    .fetch_optional(pool)
    .await
    .map_err(|_| ApiError::DatabaseInsert)?;

    let (user_id, region, credits_left, db_type) = match result {
        Some((id, reg, credits, db_type)) => (id, reg, credits, db_type),
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
                        Some(_) => return Err(ApiError::NotEnoughCredits),
                        None => return Err(ApiError::DatabaseNotFound),
                    }
                }
                None => return Err(ApiError::InvalidApiKey),
            }
        }
    };

    if credits_left <= 0 {
        return Err(ApiError::NotEnoughCredits);
    }

    Ok(UserCacheResult {
        user_id,
        region,
        db_type,
    })
}

pub async fn get_cached_user(
    state: &AppState,
    api_key: String,
    database: &str,
) -> Result<UserCacheResult, ApiError> {
    let cache_key = format!("{}_{}", api_key, database);
    if let Some(cached_result) = state.user_cache.get(&cache_key) {
        Ok(cached_result)
    } else {
        let result = validate_user_only(&state.pool, api_key, database).await?;
        state.user_cache.insert(cache_key, result.clone());
        Ok(result)
    }
}

pub async fn validate_user(
    pool: &PgPool,
    api_key: String,
    database: &str,
) -> Result<UserRegionResult, ApiError> {
    let result: Option<(i32, String)> = sqlx::query_as(
        "SELECT ak.owner_id, d.region
         FROM api_keys ak
         JOIN databases d ON d.owner_id = ak.owner_id
         WHERE ak.key = $1 AND d.name = $2",
    )
    .bind(&hash_api_key(&api_key))
    .bind(database)
    .fetch_optional(pool)
    .await
    .map_err(|_| ApiError::DatabaseInsert)?;

    let (user_id, region) = match result {
        Some((id, reg)) => (id, reg),
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
                        Some(_) => return Err(ApiError::NotEnoughCredits),
                        None => return Err(ApiError::DatabaseNotFound),
                    }
                }
                None => return Err(ApiError::InvalidApiKey),
            }
        }
    };

    Ok(UserRegionResult { user_id, region })
}

pub async fn validate_nsfw_request(
    pool: &PgPool,
    api_key: String,
) -> Result<UserNsfwValidationResult, ApiError> {
    let result: Option<(i32,)> = sqlx::query_as(
        "WITH validated_user AS (
           SELECT owner_id FROM api_keys WHERE key = $1
         ),
         updated_credits AS (
           UPDATE users 
           SET credits = credits - 1 
           WHERE id = (SELECT owner_id FROM validated_user) AND credits > 0
           RETURNING id
         )
         SELECT id FROM updated_credits",
    )
    .bind(&hash_api_key(&api_key))
    .fetch_optional(pool)
    .await
    .map_err(|_| ApiError::DatabaseInsert)?;

    let user_id = match result {
        Some((id,)) => id,
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
                Some(_) => return Err(ApiError::NotEnoughCredits),
                None => return Err(ApiError::InvalidApiKey),
            }
        }
    };

    Ok(UserNsfwValidationResult { user_id })
}

pub async fn deduct_credits(
    pool: &PgPool,
    user_id: i32,
    file_count: usize,
    database: &str,
) -> Result<i32, ApiError> {
    let result: Option<(i32,)> = sqlx::query_as(
        "WITH updated_user AS (
            UPDATE users 
            SET credits = credits - $1 
            WHERE id = $2 AND credits >= $1
            RETURNING id, credits
        )
        UPDATE databases
        SET requests = requests + 1
        WHERE name = $3 AND owner_id = $2
          AND EXISTS (SELECT 1 FROM updated_user)
        RETURNING (SELECT credits FROM updated_user)",
    )
    .bind(file_count as i32)
    .bind(user_id)
    .bind(database)
    .fetch_optional(pool)
    .await
    .map_err(|_| ApiError::Unforseen)?;

    match result {
        Some((credits_left,)) => Ok(credits_left),
        None => Err(ApiError::NotEnoughCredits),
    }
}
