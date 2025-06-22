use crate::{prelude::*, structs::search_struct::*};

pub async fn get_db_region(
    pool: &PgPool,
    database: &String,
    user_id: &i32,
) -> Result<String, SearchImageError> {
    let result: String =
        sqlx::query_scalar("SELECT region FROM databases WHERE name = $1, owner_id = $2")
            .bind(database)
            .bind(user_id)
            .fetch_one(pool)
            .await
            .map_err(|_| SearchImageError::Unforseen)?;

    Ok(result)
}
