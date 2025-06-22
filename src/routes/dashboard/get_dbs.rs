use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn get_dbs_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Database>>, DashboardError> {
    let dbs = sqlx::query_as::<_, Database>(
        "SELECT d.*, s.req_limit \
         FROM databases d \
         JOIN subscriptions s ON d.owner_id = s.user_id AND d.db_type = s.db_type \
         WHERE d.owner_id = $1",
    )
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
    let result = sqlx::query_as::<_, Database>(
        "SELECT d.*, s.req_limit \
         FROM databases d \
         JOIN subscriptions s ON d.owner_id = s.user_id AND d.db_type = s.db_type \
         WHERE d.owner_id = $1 AND d.name = $2",
    )
    .bind(&claims.user_id)
    .bind(&payload.name)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| DashboardError::Unforseen)?;

    Ok(Json(result))
}
