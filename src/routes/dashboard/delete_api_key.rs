use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn delete_api_key_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Json(payload): Json<DeleteApiKeyPayload>,
) -> Result<StatusCode, DashboardError> {
    sqlx::query("DELETE FROM api_keys WHERE owner_id = $1 AND name = $2 AND created_at = $3")
        .bind(&claims.user_id)
        .bind(&payload.name)
        .bind(&payload.created_at)
        .execute(&state.pool)
        .await
        .map_err(|_| DashboardError::Unforseen)?;

    Ok(StatusCode::OK)
}
