use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn delete_db_document_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Json(payload): Json<DocumentsPayload>,
) -> Result<StatusCode, DashboardError> {
    let result =
        sqlx::query_as::<_, Database>("SELECT * FROM databases WHERE owner_id = $1 AND name = $2")
            .bind(&claims.user_id)
            .bind(&payload.name)
            .fetch_one(&state.pool)
            .await
            .map_err(|_| DashboardError::Unforseen)?;

    let neon_pool = state
        .neon_pools
        .get_pool_by_region(&result.region)
        .ok_or(DashboardError::Unforseen)?;

    let tenant = format!("{}-{}", claims.user_id, result.name);
    sqlx::query("DELETE FROM vectors WHERE tenant = $1 AND vector_id = $2")
        .bind(&tenant)
        .bind(&payload.data)
        .fetch_all(neon_pool)
        .await
        .map_err(|_| DashboardError::Unforseen)?;

    Ok(StatusCode::OK)
}
