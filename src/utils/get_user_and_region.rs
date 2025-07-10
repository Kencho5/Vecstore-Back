use crate::prelude::*;
use crate::structs::dashboard_struct::DashboardError;

pub struct UserValidationResult {
    pub user_id: i32,
    pub region: String,
    pub credits_left: i32,
    pub db_type: String,
}

pub struct UserRegionResult {
    pub user_id: i32,
    pub region: String,
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
    let result: Option<(i32, String, i32, String)> = sqlx::query_as(
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
           RETURNING owner_id, region, db_type
         )
         SELECT ud.owner_id, ud.region, uc.credits, ud.db_type
         FROM updated_db ud
         JOIN updated_credits uc ON ud.owner_id = uc.id",
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
        db_type,
    })
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
                        Some(_) => return Err(ApiError::RequestLimitExceeded),
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

pub async fn deduct_credits(
    pool: &PgPool,
    user_id: i32,
    file_count: usize,
    database: &str,
) -> Result<i32, DashboardError> {
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
    .map_err(|e| {
        dbg!(e);
        DashboardError::Unforseen
    })?;

    match result {
        Some((credits_left,)) => Ok(credits_left),
        None => Err(DashboardError::NotEnoughCredits),
    }
}
