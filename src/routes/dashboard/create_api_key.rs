use crate::{prelude::*, structs::dashboard_struct::*};
use hex;
use rand::RngCore;

pub async fn create_api_key_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Json(payload): Json<ApiKeyPayload>,
) -> Result<StatusCode, DashboardError> {
    let mut key_bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut key_bytes);
    let api_key = hash_api_key(&hex::encode(key_bytes));

    sqlx::query("INSERT INTO api_keys(key, name, owner_id) VALUES($1, $2, $3)")
        .bind(&api_key)
        .bind(&payload.key_name)
        .bind(&claims.user_id)
        .execute(&state.pool)
        .await
        .map_err(|_| DashboardError::ApiKeyExists)?;

    Ok(StatusCode::OK)
}
