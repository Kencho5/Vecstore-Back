use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn get_dbs_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Database>>, DashboardError> {
    let dbs = sqlx::query_as::<_, Database>("SELECT * FROM databases WHERE owner_id = $1")
        .bind(&claims.user_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|_| DashboardError::Unforseen)?;

    Ok(Json(dbs))
}

pub async fn get_db_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Json(payload): Json<GetDbPayload>,
) -> Result<Json<Database>, DashboardError> {
    let mut result =
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

    let record_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM vectors WHERE tenant = $1")
        .bind(&tenant)
        .fetch_one(neon_pool)
        .await
        .map_err(|_| DashboardError::Unforseen)?;

    result.record_count = Some(record_count.0);

    Ok(Json(result))
}
