use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn get_dbs_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Database>>, DashboardError> {
    let dbs = sqlx::query_as::<_, Database>("SELECT * FROM databases WHERE owner_email = $1")
        .bind(&claims.email)
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
    let dbs = sqlx::query_as::<_, Database>(
        "SELECT * FROM databases WHERE owner_email = $1 AND name = $2",
    )
    .bind(&claims.email)
    .bind(&payload.name)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| DashboardError::Unforseen)?;

    Ok(Json(dbs))
}
