use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn delete_document_handler(
    Extension(api_key): Extension<String>,
    State(state): State<AppState>,
    Json(payload): Json<DeleteDocumentPayload>,
) -> Result<StatusCode, DashboardError> {
    let result = validate_user(&state.pool, api_key, &payload.name)
        .await
        .map_err(|_| DashboardError::Unforseen)?;

    let neon_pool = state
        .neon_pools
        .get_pool_by_region(&result.region)
        .ok_or(DashboardError::Unforseen)?;

    let tenant = format!("{}-{}", result.user_id, payload.name);
    let delete_result = sqlx::query("DELETE FROM vectors WHERE tenant = $1 AND vector_id = $2")
        .bind(&tenant)
        .bind(&payload.document_id)
        .execute(neon_pool)
        .await
        .map_err(|_| DashboardError::Unforseen)?;

    if delete_result.rows_affected() == 0 {
        return Err(DashboardError::NotFound);
    }

    Ok(StatusCode::OK)
}
