use crate::{prelude::*, structs::payment_struct::*};

pub async fn payment_created_handler(
    State(state): State<AppState>,
    Json(payload): Json<PaymentCreatedPayload>,
) -> Result<StatusCode, PaymentError> {
    let data = &payload.data;

    let email = data
        .custom_data
        .as_ref()
        .and_then(|cd| cd.user_email.as_ref())
        .ok_or(PaymentError::MissingCustomerData)?;

    let first_item = data.items.first().ok_or(PaymentError::Unforseen)?;
    let plan_name = &first_item.product.name;
    let price = &first_item.price.unit_price.amount;

    sqlx::query(
        "INSERT INTO subscriptions(
            user_email, customer_id, subscription_id, plan_name, price, status, next_billing_date
        ) VALUES ($1, $2, $3, $4, $5, $6, $7::timestamptz::date)",
    )
    .bind(email)
    .bind(&data.customer_id)
    .bind(&data.id)
    .bind(plan_name)
    .bind(price)
    .bind(&data.status)
    .bind(&data.next_billed_at)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        PaymentError::Unforseen
    })?;

    Ok(StatusCode::OK)
}
