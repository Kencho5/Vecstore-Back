use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn add_db_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Json(payload): Json<AddDbPayload>,
) -> Result<StatusCode, DashboardError> {
    payload
        .validate()
        .map_err(|_| DashboardError::MissingData)?;

    let result = sqlx::query(
        "
        INSERT INTO databases(name, db_type, region, owner_id)
        VALUES($1, $2, $3, $4)
        ",
    )
    .bind(&payload.name.to_lowercase())
    .bind(&payload.db_type)
    .bind(&payload.region)
    .bind(&claims.user_id)
    .bind(&payload.db_type)
    .execute(&state.pool)
    .await
    .map_err(|_| DashboardError::DatabaseExists)?;

    if result.rows_affected() == 0 {
        return Err(DashboardError::MissingSubscription);
    }

    Ok(StatusCode::OK)
}
