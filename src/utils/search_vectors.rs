use crate::prelude::*;
use sqlx::Row;

pub async fn search_vectors_with_region(
    state: &AppState,
    vectors: Vec<f32>,
    user_id: i32,
    database: &String,
    region: &String,
    metadata_filter: Option<serde_json::Value>,
) -> Result<SearchResults, ApiError> {
    let pool = state
        .neon_pools
        .get_pool_by_region(region)
        .ok_or(ApiError::Unforseen)?;

    search_vectors_impl(vectors, user_id, database, pool, metadata_filter).await
}

async fn search_vectors_impl(
    vectors: Vec<f32>,
    user_id: i32,
    database: &String,
    pool: &PgPool,
    metadata_filter: Option<serde_json::Value>,
) -> Result<SearchResults, ApiError> {
    let tenant = format!("{}-{}", user_id, database);

    let rows = if let Some(metadata_json) = metadata_filter {
        sqlx::query(
            "SELECT vector_id, embedding <=> $1::vector AS distance, metadata FROM vectors WHERE tenant = $2 AND metadata @> $3 ORDER BY distance LIMIT 3"
        )
        .bind(&vectors)
        .bind(&tenant)
        .bind(&metadata_json)
        .fetch_all(pool)
        .await
    } else {
        sqlx::query(
            "SELECT vector_id, embedding <=> $1::vector AS distance, metadata FROM vectors WHERE tenant = $2 ORDER BY distance LIMIT 3"
        )
        .bind(&vectors)
        .bind(&tenant)
        .fetch_all(pool)
        .await
    }
    .map_err(|_| {
        ApiError::DatabaseError
    })?;

    let matches = rows
        .into_iter()
        .map(|row| {
            let vector_id: String = row.get("vector_id");
            let distance: f64 = row.get("distance");
            let metadata: Option<serde_json::Value> = row.get("metadata");

            // Convert distance to similarity score (1 - distance, clamped to 0-1)
            let similarity = (1.0 - distance).max(0.0).min(1.0);

            let metadata_map = metadata.and_then(|m| {
                m.as_object().map(|obj| {
                    obj.iter()
                        .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                        .collect::<std::collections::HashMap<String, String>>()
                })
            });

            SearchMatch {
                id: vector_id,
                score: format!("{:.2}%", similarity * 100.0),
                metadata: metadata_map,
            }
        })
        .collect();

    Ok(SearchResults { matches })
}
