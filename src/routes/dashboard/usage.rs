use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn usage_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> Result<Json<UsageResponse>, DashboardError> {
    let result: Option<i32> =
        sqlx::query_scalar("SELECT SUM(requests) FROM databases WHERE owner_id = $1")
            .bind(&claims.user_id)
            .fetch_one(&state.pool)
            .await
            .map_err(|_| DashboardError::Unforseen)?;

    Ok(Json(UsageResponse {
        requests: result.unwrap_or(0),
    }))
}
