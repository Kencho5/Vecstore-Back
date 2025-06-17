use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn payment_methods_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, DashboardError> {
    let payment_methods = state
        .paddle
        .payment_methods_list("ctm_01jk84f1s981kf2a4fqmv968ba");

    let mut paginated = payment_methods.send();
    let mut all_methods = Vec::new();

    while let Ok(Some(page_result)) = paginated.next().await {
        let page = page_result; // or just `?` if your error types match
        all_methods.extend(page.data);
    }

    Ok(Json(all_methods))
}
