use crate::prelude::*;

pub async fn payment_failed_handler(
    State(state): State<AppState>,
    Json(payload): Json<PaymentFailedPayload>,
) -> Result<StatusCode, PaymentError> {
    let data = &payload.data;

    let result = sqlx::query(
        "UPDATE subscriptions 
         SET status = $2, 
             updated_at = CURRENT_TIMESTAMP 
         WHERE subscription_id = $1",
    )
    .bind(&data.id)
    .bind(&data.status)
    .execute(&state.pool)
    .await
    .map_err(|_| PaymentError::Unforseen)?;

    if result.rows_affected() == 0 {
        return Err(PaymentError::Unforseen);
    }

    Ok(StatusCode::OK)
}

