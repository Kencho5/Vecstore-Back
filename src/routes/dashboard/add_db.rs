use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn add_db_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Json(payload): Json<AddDbPayload>,
) -> Result<StatusCode, DashboardError> {
    payload
        .validate()
        .map_err(|_| DashboardError::MissingDbData)?;

    sqlx::query("INSERT INTO databases(name, db_type, region, owner_id) VALUES($1, $2, $3, $4)")
        .bind(&payload.name)
        .bind(&payload.db_type)
        .bind(&payload.region)
        .bind(&claims.user_id)
        .execute(&state.pool)
        .await
        .map_err(|error| {
            println!("Add DB Error: {:?}", error);
            DashboardError::DatabaseExists
        })?;

    Ok(StatusCode::OK)
}
