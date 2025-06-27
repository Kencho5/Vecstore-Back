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
) -> Result<UserValidationResult, InsertError> {
    let db_info_result: Option<(i32, String)> = sqlx::query_as(
        "SELECT ak.owner_id, d.db_type 
         FROM api_keys ak
         JOIN databases d ON d.owner_id = ak.owner_id
         WHERE ak.key = $1 AND d.name = $2",
    )
    .bind(&hash_api_key(&api_key))
    .bind(&database)
    .fetch_optional(pool)
    .await
    .map_err(|_| InsertError::InvalidApiKey)?;

    let (user_id, db_type) = match db_info_result {
        Some((id, db_type)) => (id, db_type),
        None => return Err(InsertError::DatabaseNotFound),
    };

    let db_result: Option<(String, i32)> = sqlx::query_as(
        "UPDATE databases 
         SET requests = requests + 1 
         WHERE name = $1 AND owner_id = $2
         RETURNING region, requests - 1 as previous_requests",
    )
    .bind(&database)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|_| InsertError::DatabaseNotFound)?;

    let (region, _previous_requests) = match db_result {
        Some((reg, _prev)) => (reg, _prev),
        None => {
            let exists: Option<(i32,)> =
                sqlx::query_as("SELECT requests FROM databases WHERE name = $1 AND owner_id = $2")
                    .bind(&database)
                    .bind(user_id)
                    .fetch_optional(pool)
                    .await
                    .map_err(|_| InsertError::DatabaseNotFound)?;

            match exists {
                Some(_) => return Err(InsertError::DatabaseNotFound),
                None => return Err(InsertError::DatabaseNotFound),
            }
        }
    };

    Ok(UserValidationResult { user_id, region })
}
