use crate::{
    prelude::*,
    structs::dashboard_struct::{DashboardError, PortalUrlBody},
};

pub async fn portal_url_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> Result<Json<PortalUrlBody>, DashboardError> {
    let customer_id = get_customer_id(claims.email, &state.pool)
        .await
        .map_err(|_| DashboardError::Unauthorized)?;

    let portal_url = state
        .paddle
        .create_portal_session(customer_id)
        .send()
        .await
        .unwrap();

    Ok(Json(PortalUrlBody {
        url: portal_url.data.urls.general.overview,
    }))
}
