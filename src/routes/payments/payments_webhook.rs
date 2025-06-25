use crate::prelude::*;
use crate::routes::payments::subscription_created::*;

pub async fn payments_webhook_handler(
    State(state): State<AppState>,
    Json(payload): Json<PaymentWebhookPayload>,
) -> Result<StatusCode, PaymentError> {
    match payload.event_type.as_str() {
        "subscription.created" => subscription_created(&state, &payload).await?,
        "subscription.updated" => {
            dbg!(&payload);
        }
        "subscription.past_due" => {}
        "subscription.canceled" => {}
        _ => {}
    }
    Ok(StatusCode::OK)
}
