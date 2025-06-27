use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn usage_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> Result<Json<Vec<UsageLogsResponse>>, DashboardError> {
    let result = sqlx::query_as::<_, UsageLogsResponse>(
        "SELECT * FROM usage_logs WHERE user_id = $1 ORDER BY usage_date ASC",
    )
    .bind(&claims.user_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|_| DashboardError::Unforseen)?;

    Ok(Json(result))
}
