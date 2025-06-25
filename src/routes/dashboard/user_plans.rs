use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn user_plans_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> Result<Json<Vec<String>>, DashboardError> {
    let plan_names = sqlx::query_scalar::<_, String>(
        "SELECT db_type FROM subscriptions WHERE user_id = $1 AND status = 'active'",
    )
    .bind(&claims.user_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|_| DashboardError::Unforseen)?;

    Ok(Json(plan_names))
}
