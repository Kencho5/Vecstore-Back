use crate::{prelude::*, structs::payment_struct::*};

pub async fn payment_created_handler(
    State(state): State<AppState>,
    Json(payload): Json<PaymentCreatedPayload>,
) -> Result<StatusCode, PaymentError> {
    println!("{:?}", payload.event_id);
    Ok(StatusCode::OK)
}
