use crate::prelude::*;

pub async fn subscription_canceled_handler(
    State(state): State<AppState>,
    Json(payload): Json<SubscriptionCanceledPayload>,
) -> Result<StatusCode, PaymentError> {
    let data = &payload.data;

    let result = sqlx::query(
        "UPDATE subscriptions 
         SET status = $1, 
             next_billing_date = NULL,
             updated_at = CURRENT_TIMESTAMP 
         WHERE subscription_id = $2",
    )
    .bind(&data.status)
    .bind(&data.id)
    .execute(&state.pool)
    .await
    .map_err(|_| PaymentError::Unforseen)?;

    if result.rows_affected() == 0 {
        return Err(PaymentError::Unforseen);
    }

    Ok(StatusCode::OK)
}
