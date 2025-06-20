use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn usage_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> Result<Json<Vec<UsageResponse>>, DashboardError> {
    let result = sqlx::query_as::<_, UsageResponse>(
        "
        SELECT
          s.plan_name,
          s.req_limit,
          COALESCE(SUM(d.requests), 0) AS used_requests
        FROM subscriptions s
        LEFT JOIN databases d
          ON d.owner_id = s.user_id
          AND (
            (s.plan_name = 'Image Search' AND d.db_type = 'image') OR
            (s.plan_name = 'Text Search' AND d.db_type = 'text')
          )
        WHERE s.user_id = $1
        GROUP BY s.plan_name, s.req_limit
        ",
    )
    .bind(&claims.user_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|_| DashboardError::Unforseen)?;

    Ok(Json(result))
}
