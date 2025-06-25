use crate::prelude::*;

pub async fn subscription_updated(
    state: &AppState,
    payload: &PaymentWebhookPayload,
) -> Result<(), PaymentError> {
    let data = &payload.data;

    let subscription_id = &data.id;
    let customer_id = &data.customer_id;
    let status = &data.status;

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
        UPDATE subscriptions SET 
            plan_name = $1, plan_type = $2, db_type = $3, price = $4, 
            req_limit = $5, status = $6, next_billing_date = $7
        WHERE subscription_id = $8 AND customer_id = $9
        "#,
    )
    .bind(plan_name)
    .bind(plan_type)
    .bind(db_type)
    .bind(amount)
    .bind(req_limit)
    .bind(status)
    .bind(next_billing_date)
    .bind(subscription_id)
    .bind(customer_id)
    .execute(&state.pool)
    .await
    .map_err(|_| PaymentError::Unforseen)?;

    Ok(())
}
