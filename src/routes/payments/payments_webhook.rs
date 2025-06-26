use crate::prelude::*;
use crate::routes::payments::transaction_completed::*;
use crate::routes::payments::transaction_failed::*;

pub async fn payments_webhook_handler(
    State(state): State<AppState>,
    Json(payload): Json<PaymentWebhookPayload>,
) -> Result<StatusCode, PaymentError> {
    match payload.event_type.as_str() {
        "transaction.completed" => transaction_completed(&state, &payload).await?,
        "transaction.payment_failed" => transaction_failed(&state, &payload).await?,
        _ => {}
    }
    Ok(StatusCode::OK)
}
