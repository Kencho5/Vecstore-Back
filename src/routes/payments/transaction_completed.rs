use crate::prelude::*;

pub async fn transaction_completed(
    state: &AppState,
    payload: &PaymentWebhookPayload,
) -> Result<(), PaymentError> {
    let data = &payload.data;
    let price = &data.items[0].price;

    let user_id = data.custom_data.user_id as i32;
    let credits_purchased = price
        .custom_data
        .credits
        .parse::<i32>()
        .map_err(|_| PaymentError::MissingCustomerData)?;
    let amount_paid = price
        .unit_price
        .amount
        .parse::<i32>()
        .map_err(|_| PaymentError::MissingCustomerData)?;
    let billed_at = match &data.billed_at {
        Some(billed_at_str) => Some(
            DateTime::parse_from_rfc3339(billed_at_str)
                .map_err(|_| PaymentError::MissingCustomerData)?
                .naive_utc(),
        ),
        None => None,
    };
    let payment_method = &data.payments[0].method_details.payment_type;

    sqlx::query(
        r#"
        INSERT INTO transactions (
            user_id, user_email, transaction_id, customer_id, price_id,
            plan_name, plan_description, credits_purchased, amount_paid,
            status, billed_at, invoice_id, invoice_number, payment_method
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        "#,
    )
    .bind(user_id)
    .bind(&data.custom_data.user_email)
    .bind(&data.id)
    .bind(&data.customer_id)
    .bind(&price.id)
    .bind(&price.name)
    .bind(&price.description)
    .bind(credits_purchased)
    .bind(amount_paid)
    .bind(&data.status)
    .bind(billed_at)
    .bind(&data.invoice_id)
    .bind(&data.invoice_number)
    .bind(payment_method)
    .execute(&state.pool)
    .await
    .map_err(|_| PaymentError::Unforseen)?;

    sqlx::query("UPDATE users SET credits = credits + $1 WHERE id = $2")
        .bind(&credits_purchased)
        .bind(user_id)
        .execute(&state.pool)
        .await
        .map_err(|_| PaymentError::Unforseen)?;

    Ok(())
}
