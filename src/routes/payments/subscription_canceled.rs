use crate::prelude::*;

pub async fn subscription_canceled_handler(
    State(state): State<AppState>,
    Json(payload): Json<PaymentCreatedPayload>,
) -> Result<StatusCode, PaymentError> {
    Ok(StatusCode::OK)
}
