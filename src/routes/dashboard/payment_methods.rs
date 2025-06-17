use crate::{prelude::*, structs::dashboard_struct::*};
use paddle_rust_sdk::entities::Card;

pub async fn payment_methods_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Card>>, DashboardError> {
    let customer_id = get_customer_id(claims.email, &state.pool)
        .await
        .map_err(|_| DashboardError::Unforseen)?;
    let payment_methods = state.paddle.payment_methods_list(customer_id);

    let mut paginated = payment_methods.send();
    let mut all_methods = Vec::new();

    while let Ok(Some(page_result)) = paginated.next().await {
        let page = page_result;
        all_methods.extend(page.data);
    }

    let cards: Vec<Card> = all_methods.into_iter().filter_map(|pm| pm.card).collect();

    Ok(Json(cards))
}
