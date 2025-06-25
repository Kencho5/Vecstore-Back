use crate::{prelude::*, structs::dashboard_struct::*};
use paddle_rust_sdk::enums::ProrationBillingMode;
use paddle_rust_sdk::transactions::TransactionItem;

pub async fn upgrade_subscription_handler(
    State(state): State<AppState>,
    Json(payload): Json<SubscriptionPreviewPayload>,
) -> Result<StatusCode, DashboardError> {
    let price_maps = get_price_maps();
    let current_subscription = state
        .paddle
        .subscription_get(&payload.subscription_id)
        .send()
        .await
        .unwrap();

    let current_price_id = &current_subscription.data.items.first().unwrap().price.id.0;
    let pro_plan_price_id = price_maps.get(current_price_id).unwrap();

    let new_item = TransactionItem::CatalogItem {
        price_id: paddle_rust_sdk::ids::PriceID(pro_plan_price_id.to_string()),
        quantity: 1,
    };

    state
        .paddle
        .subscription_update(&payload.subscription_id)
        .items([new_item])
        .proration_billing_mode(ProrationBillingMode::FullImmediately)
        .send()
        .await
        .map_err(|_| DashboardError::Unforseen)?;

    Ok(StatusCode::OK)
}

pub async fn preview_subscription_handler(
    State(state): State<AppState>,
    Json(payload): Json<SubscriptionPreviewPayload>,
) -> Result<Json<SubscriptionPreview>, DashboardError> {
    let price_maps = get_price_maps();
    let current_subscription = state
        .paddle
        .subscription_get(&payload.subscription_id)
        .send()
        .await
        .unwrap();

    let current_price_id = &current_subscription.data.items.first().unwrap().price.id.0;
    let pro_plan_price_id = price_maps.get(current_price_id).unwrap();

    let new_item = TransactionItem::CatalogItem {
        price_id: paddle_rust_sdk::ids::PriceID(pro_plan_price_id.to_string()),
        quantity: 1,
    };

    let response = state
        .paddle
        .subscription_preview_update(&payload.subscription_id)
        .items([new_item])
        .proration_billing_mode(ProrationBillingMode::FullImmediately)
        .send()
        .await
        .unwrap();

    let data = response.data;

    let preview = SubscriptionPreview {
        subscription_id: payload.subscription_id,
        next_billed_at: data.next_billed_at.unwrap(),
        amount: data
            .update_summary
            .unwrap()
            .charge
            .amount
            .parse::<i32>()
            .unwrap()
            / 100,
        plan_name: data
            .items
            .first()
            .and_then(|item| item.price.name.clone())
            .unwrap_or_default(),
    };

    Ok(Json(preview))
}
