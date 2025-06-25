use crate::prelude::*;

pub async fn subscription_created(
    state: &AppState,
    payload: &PaymentWebhookPayload,
) -> Result<(), PaymentError> {
    let data = &payload.data;

    let subscription_id = &data.id;
    let customer_id = &data.customer_id;
    let status = &data.status;
    let user_id = data.custom_data.user_id as i32;
    let user_email = &data.custom_data.user_email;

    let price = &data.items[0].price;
    let plan_name = &price.name;
    let plan_type = &price.description;
    let db_type = &price.custom_data.db_type;
    let req_limit = price
        .custom_data
        .limit
        .parse::<i32>()
        .map_err(|_| PaymentError::MissingCustomerData)?;
    let amount = price
        .unit_price
        .amount
        .parse::<i32>()
        .map_err(|_| PaymentError::MissingCustomerData)?
        / 100;

    let next_billing_date = DateTime::parse_from_rfc3339(&data.next_billed_at)
        .map_err(|_| PaymentError::MissingCustomerData)?
        .naive_utc()
        .date();

    sqlx::query(
        r#"
        INSERT INTO subscriptions (
            user_id, user_email, customer_id, subscription_id, 
            plan_name, plan_type, db_type, price, req_limit, 
            status, next_billing_date
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
    )
    .bind(user_id)
    .bind(user_email)
    .bind(customer_id)
    .bind(subscription_id)
    .bind(plan_name)
    .bind(plan_type)
    .bind(db_type)
    .bind(amount)
    .bind(req_limit)
    .bind(status)
    .bind(next_billing_date)
    .execute(&state.pool)
    .await
    .map_err(|_| PaymentError::Unforseen)?;

    Ok(())
}
