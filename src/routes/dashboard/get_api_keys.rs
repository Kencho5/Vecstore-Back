use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn get_api_keys_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> Result<Json<Vec<ApiKeysResponse>>, DashboardError> {
    let keys = sqlx::query_as::<_, ApiKeysResponse>(
        "SELECT name, created_at FROM api_keys WHERE owner_id = $1",
    )
    .bind(&claims.user_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|_| DashboardError::Unforseen)?;

    Ok(Json(keys))
}
