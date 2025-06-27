use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn user_credits_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> Result<Json<UserCredits>, DashboardError> {
    let user_credits = sqlx::query_as::<_, UserCredits>("SELECT credits FROM users WHERE id = $1")
        .bind(&claims.user_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| DashboardError::Unforseen)?;

    Ok(Json(user_credits))
}
