use crate::{prelude::*, structs::dashboard_struct::*};
use hex;
use rand::RngCore;

pub async fn create_api_key_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Json(payload): Json<ApiKeyPayload>,
) -> Result<Json<ApiKeyResponse>, DashboardError> {
    payload
        .validate()
        .map_err(|_| DashboardError::MissingData)?;

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM api_keys WHERE owner_id = $1")
        .bind(&claims.user_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| DashboardError::Unforseen)?;

    if count >= 10 {
        return Err(DashboardError::ApiKeyCreationLimit);
    }

    let mut key_bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut key_bytes);
    let api_key = hex::encode(key_bytes);
    let hashed_api_key = hash_api_key(&api_key);

    sqlx::query("INSERT INTO api_keys(key, name, owner_id) VALUES($1, $2, $3)")
        .bind(&hashed_api_key)
        .bind(&payload.key_name)
        .bind(&claims.user_id)
        .execute(&state.pool)
        .await
        .map_err(|_| DashboardError::ApiKeyExists)?;

    Ok(Json(ApiKeyResponse { key: api_key }))
}
