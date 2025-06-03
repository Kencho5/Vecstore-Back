use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn add_db_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Json(payload): Json<AddDbPayload>,
) -> Result<StatusCode, DashboardError> {
    payload
        .validate()
        .map_err(|_| DashboardError::MissingDbData)?;

    sqlx::query("INSERT INTO databases(name, db_type, region, owner_email) VALUES($1, $2, $3, $4)")
        .bind(&payload.name)
        .bind(&payload.db_type)
        .bind(&payload.region)
        .bind(&claims.email)
        .execute(&state.pool)
        .await
        .map_err(|_| DashboardError::DatabaseExists)?;

    Ok(StatusCode::OK)
}
