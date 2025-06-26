use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn list_transactions_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Transaction>>, DashboardError> {
    let dbs = sqlx::query_as::<_, Transaction>("SELECT * FROM transactions WHERE user_id = $1")
        .bind(&claims.user_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|_| DashboardError::Unforseen)?;

    Ok(Json(dbs))
}
