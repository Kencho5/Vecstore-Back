use crate::prelude::*;
use sqlx::Error;

pub async fn get_customer_id(email: String, pool: &PgPool) -> Result<String, Error> {
    let customer_id: String =
        sqlx::query_scalar("SELECT customer_id FROM transactions WHERE user_email = $1")
            .bind(email)
            .fetch_one(pool)
            .await?;

    Ok(customer_id)
}
