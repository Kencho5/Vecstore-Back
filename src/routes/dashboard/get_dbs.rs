use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn get_dbs_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> Result<Json<Vec<AddDbPayload>>, DashboardError> {
    let dbs = sqlx::query_as::<_, AddDbPayload>("SELECT * FROM databases WHERE owner_email = $1")
        .bind(&claims.email)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| {
            println!("{:?}", e);
            DashboardError::Unforseen
        })?;

    Ok(Json(dbs))
}
