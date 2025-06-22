use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn list_subscriptions_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Subscription>>, DashboardError> {
    let user_subs =
        sqlx::query_as::<_, Subscription>("SELECT subscription_id, plan_name, plan_type, price, status, next_billing_date FROM subscriptions WHERE user_email = $1 AND status = 'active' OR status = 'past_due'")
            .bind(&claims.email)
            .fetch_all(&state.pool)
            .await
            .map_err(|_| DashboardError::Unforseen)?;

    Ok(Json(user_subs))
}
