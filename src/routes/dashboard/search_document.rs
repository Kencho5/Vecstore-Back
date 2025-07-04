use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn search_document_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Json(payload): Json<DocumentsPayload>,
) -> Result<Json<Vec<DatabaseDocument>>, DashboardError> {
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
    let documents = sqlx::query_as::<_, DatabaseDocument>(
        "SELECT vector_id, metadata, created_at FROM vectors WHERE tenant = $1 AND vector_id = $2 LIMIT 5",
    )
    .bind(&tenant)
    .bind(&payload.vector_id)
    .fetch_all(neon_pool)
    .await
    .map_err(|_| DashboardError::Unforseen)?;

    Ok(Json(documents))
}
