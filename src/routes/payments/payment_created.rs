use crate::prelude::*;

pub async fn payment_created_handler(
    State(state): State<AppState>,
    Json(payload): Json<PaymentCreatedPayload>,
) -> Result<StatusCode, PaymentError> {
    let data = &payload.data;

    let user_id = data
        .custom_data
        .as_ref()
        .and_then(|cd| cd.user_id.as_ref())
        .ok_or(PaymentError::MissingCustomerData)?;

    let email = data
        .custom_data
        .as_ref()
        .and_then(|cd| cd.user_email.as_ref())
        .ok_or(PaymentError::MissingCustomerData)?;

    let first_item = data.items.first().ok_or(PaymentError::Unforseen)?;
    let plan_name = &first_item.product.name;
    let price = (&first_item.price.unit_price.amount.parse::<i32>().unwrap()) / 100;
    let limit = &first_item.price.custom_data.limit.parse::<i32>().unwrap();
    let db_type = &first_item.price.custom_data.db_type;
    let plan_type = &first_item.price.description;

    sqlx::query(
        "INSERT INTO subscriptions(
            user_id, user_email, customer_id, subscription_id, plan_name, plan_type, db_type, price, req_limit, status, next_billing_date
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11::timestamptz::date)",
    )
    .bind(user_id)
    .bind(email)
    .bind(&data.customer_id)
    .bind(&data.id)
    .bind(plan_name)
    .bind(plan_type)
    .bind(db_type)
    .bind(price)
    .bind(limit)
    .bind(&data.status)
    .bind(&data.next_billed_at)
    .execute(&state.pool)
    .await
    .map_err(|_| PaymentError::Unforseen)?;

    Ok(StatusCode::OK)
}
