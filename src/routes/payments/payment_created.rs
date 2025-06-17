use crate::{prelude::*, structs::payment_struct::*};

pub async fn payment_created_handler(
    State(state): State<AppState>,
    Json(payload): Json<PaymentCreatedPayload>,
) -> Result<StatusCode, PaymentError> {
    let subscription_id = payload.data.id;
    let status = payload.data.status;
    let next_billing_date = payload.data.next_billed_at;

    let plan_name = payload
        .data
        .items
        .get(0)
        .map(|item| item.product.name.clone())
        .unwrap_or_default();

    let email = payload
        .data
        .items
        .get(0)
        .and_then(|item| item.product.custom_data.as_ref())
        .and_then(|custom_data| custom_data.email.clone());

    sqlx::query(
        "INSERT INTO subscriptions(
        user_email, subscription_id, plan_name, status, next_billing_date
    ) VALUES ($1, $2, $3, $4, $5::timestamptz::date)",
    )
    .bind(&email)
    .bind(&subscription_id)
    .bind(&plan_name)
    .bind(&status)
    .bind(&next_billing_date)
    .execute(&state.pool)
    .await
    .map_err(|_| PaymentError::Unforseen)?;

    Ok(StatusCode::OK)
}
