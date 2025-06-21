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
        SELECT $1, $2, $3, $4
        WHERE EXISTS (
          SELECT 1 FROM subscriptions
          WHERE user_id = $4 AND plan_name = $5 AND status = 'active'
        );
        ",
    )
    .bind(&payload.name.to_lowercase())
    .bind(&payload.db_type)
    .bind(&payload.region)
    .bind(&claims.user_id)
    .bind(&format!("{} Search", capitalize(&payload.db_type)))
    .execute(&state.pool)
    .await
    .map_err(|_| DashboardError::DatabaseExists)?;

    if result.rows_affected() == 0 {
        return Err(DashboardError::MissingSubscription);
    }

    Ok(StatusCode::OK)
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
