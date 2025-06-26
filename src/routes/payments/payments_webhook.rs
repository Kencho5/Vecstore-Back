use crate::prelude::*;
use crate::routes::payments::transaction_completed::*;

pub async fn payments_webhook_handler(
    State(state): State<AppState>,
    Json(payload): Json<PaymentWebhookPayload>,
) -> Result<StatusCode, PaymentError> {
    match payload.event_type.as_str() {
        "transaction.completed" => transaction_completed(&state, &payload).await?,
        _ => {}
    }
    Ok(StatusCode::OK)
}
