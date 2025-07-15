use crate::{prelude::*, structs::dashboard_struct::DashboardError};

pub async fn save_usage_logs(pool: PgPool, user_id: i32, count: i32) -> Result<(), DashboardError> {
    let today = Utc::now().date_naive();

    sqlx::query(
        r#"
        INSERT INTO usage_logs (user_id, usage_date, credits_used)
        VALUES ($1, $2, 1)
        ON CONFLICT (user_id, usage_date)
        DO UPDATE SET
            credits_used = usage_logs.credits_used + $3
        "#,
    )
    .bind(user_id)
    .bind(today)
    .bind(count)
    .execute(&pool)
    .await
    .map_err(|_| DashboardError::Unforseen)?;

    sqlx::query(
        r#"
        DELETE FROM usage_logs 
        WHERE usage_date < CURRENT_DATE - INTERVAL '7 days'
        "#,
    )
    .execute(&pool)
    .await
    .map_err(|_| DashboardError::Unforseen)?;

    Ok(())
}
