use crate::prelude::*;
use sqlx::Row;

pub async fn search_vectors(
    state: &AppState,
    query_text: &str,
    vectors: Vec<f32>,
    user_id: i32,
    database: &String,
    region: &String,
    metadata_filter: Option<serde_json::Value>,
    page: Option<u32>,
    limit: Option<u32>,
) -> Result<SearchResults, ApiError> {
    let pool = state
        .neon_pools
        .get_pool_by_region(region)
        .ok_or(ApiError::Unforseen)?;

    let tenant = format!("{}-{}", user_id, database);
    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(10).min(100) as i64;
    let offset = ((page - 1) * (limit as u32)) as i64;
    let distance_threshold = 0.8;

    let use_hybrid = !query_text.trim().is_empty();

    if use_hybrid {
        hybrid_search_query(
            query_text,
            vectors,
            tenant,
            pool,
            metadata_filter,
            distance_threshold,
            limit,
            offset,
        )
        .await
    } else {
        vector_search_query(
            vectors,
            tenant,
            pool,
            metadata_filter,
            distance_threshold,
            limit,
            offset,
        )
        .await
    }
}

fn build_hybrid_search_query(has_metadata_filter: bool) -> &'static str {
    if has_metadata_filter {
        r#"
       WITH query_cache AS (
           SELECT plainto_tsquery('simple', $5) as tsquery
       ),
       vector_candidates AS (
           SELECT vector_id, embedding <=> $1::vector AS distance, metadata, content, search_vector
           FROM vectors 
           WHERE tenant = $2 AND metadata @> $3 
           AND embedding <=> $1::vector < $4 + 0.2
       ),
       text_candidates AS (
           SELECT vector_id, embedding <=> $1::vector AS distance, metadata, content, search_vector
           FROM vectors 
           CROSS JOIN query_cache
           WHERE tenant = $2 AND metadata @> $3 
           AND (
               content ILIKE '%' || $5 || '%' OR
               search_vector @@ query_cache.tsquery
           )
       ),
       all_candidates AS (
           SELECT * FROM vector_candidates
           UNION
           SELECT * FROM text_candidates
       ),
       scored_results AS (
           SELECT vector_id, distance, metadata, content,
                  CASE WHEN search_vector @@ query_cache.tsquery
                      THEN ts_rank_cd(search_vector, query_cache.tsquery) * 0.4
                      ELSE 0 END +
                  CASE WHEN word_similarity($5, content) > 0.1
                      THEN word_similarity($5, content) * 0.3
                      ELSE 0 END +
                  CASE WHEN content ILIKE '%' || $5 || '%'
                      THEN 0.25
                      ELSE 0 END as text_score
           FROM all_candidates
           CROSS JOIN query_cache
       ),
       combined_scores AS (
           SELECT vector_id, distance, metadata, content,
                  (1.0 - distance) * 0.6 as vector_score,
                  text_score,
                  LEAST(1.0, (1.0 - distance) * 0.6 + text_score) as combined_score
           FROM scored_results
           WHERE distance < $4 OR text_score > 0.1
       )
       SELECT vector_id, distance, metadata, content, combined_score, vector_score, text_score
       FROM combined_scores
       ORDER BY combined_score DESC
       LIMIT $6 OFFSET $7
       "#
    } else {
        r#"
       WITH query_cache AS (
           SELECT plainto_tsquery('simple', $4) as tsquery
       ),
       vector_candidates AS (
           SELECT vector_id, embedding <=> $1::vector AS distance, metadata, content, search_vector
           FROM vectors 
           WHERE tenant = $2 
           AND embedding <=> $1::vector < $3 + 0.2
       ),
       text_candidates AS (
           SELECT vector_id, embedding <=> $1::vector AS distance, metadata, content, search_vector
           FROM vectors 
           CROSS JOIN query_cache
           WHERE tenant = $2 
           AND (
               content ILIKE '%' || $4 || '%' OR
               search_vector @@ query_cache.tsquery
           )
       ),
       all_candidates AS (
           SELECT * FROM vector_candidates
           UNION
           SELECT * FROM text_candidates
       ),
       scored_results AS (
           SELECT vector_id, distance, metadata, content,
                  CASE WHEN search_vector @@ query_cache.tsquery
                      THEN ts_rank_cd(search_vector, query_cache.tsquery) * 0.4
                      ELSE 0 END +
                  CASE WHEN word_similarity($4, content) > 0.1
                      THEN word_similarity($4, content) * 0.3
                      ELSE 0 END +
                  CASE WHEN content ILIKE '%' || $4 || '%'
                      THEN 0.25
                      ELSE 0 END as text_score
           FROM all_candidates
           CROSS JOIN query_cache
       ),
       combined_scores AS (
           SELECT vector_id, distance, metadata, content,
                  (1.0 - distance) * 0.6 as vector_score,
                  text_score,
                  LEAST(1.0, (1.0 - distance) * 0.6 + text_score) as combined_score
           FROM scored_results
           WHERE distance < $3 OR text_score > 0.1
       )
       SELECT vector_id, distance, metadata, content, combined_score, vector_score, text_score
       FROM combined_scores
       ORDER BY combined_score DESC
       LIMIT $5 OFFSET $6
       "#
    }
}

async fn hybrid_search_query(
    query_text: &str,
    vectors: Vec<f32>,
    tenant: String,
    pool: &PgPool,
    metadata_filter: Option<serde_json::Value>,
    distance_threshold: f64,
    limit: i64,
    offset: i64,
) -> Result<SearchResults, ApiError> {
    let sql = build_hybrid_search_query(metadata_filter.is_some());

    let rows = if let Some(metadata_json) = metadata_filter {
        sqlx::query(sql)
            .bind(&vectors)
            .bind(&tenant)
            .bind(&metadata_json)
            .bind(distance_threshold)
            .bind(query_text)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
    } else {
        sqlx::query(sql)
            .bind(&vectors)
            .bind(&tenant)
            .bind(distance_threshold)
            .bind(query_text)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
    }
    .map_err(|e| {
        eprintln!("Database error in hybrid search: {}", e);
        ApiError::DatabaseError
    })?;

    let matches = rows
        .into_iter()
        .map(|row| {
            let vector_id: String = row.get("vector_id");
            let combined_score: f64 = row.get("combined_score");
            let content: Option<String> = row.get("content");
            let metadata: Option<serde_json::Value> = row.get("metadata");

            SearchResult {
                vector_id,
                content,
                metadata,
                score: Some(format!("{:.1}%", combined_score * 100.0)),
            }
        })
        .collect();

    Ok(SearchResults { matches })
}

async fn vector_search_query(
    vectors: Vec<f32>,
    tenant: String,
    pool: &PgPool,
    metadata_filter: Option<serde_json::Value>,
    distance_threshold: f64,
    limit: i64,
    offset: i64,
) -> Result<SearchResults, ApiError> {
    let rows = if let Some(metadata_json) = metadata_filter {
       sqlx::query(
           "SELECT vector_id, content, embedding <=> $1::vector AS distance, metadata FROM vectors WHERE tenant = $2 AND metadata @> $3 AND embedding <=> $1::vector < $4 ORDER BY embedding <=> $1::vector LIMIT $5 OFFSET $6"
       )
       .bind(&vectors)
       .bind(&tenant)
       .bind(&metadata_json)
       .bind(distance_threshold)
       .bind(limit)
       .bind(offset)
       .fetch_all(pool)
       .await
   } else {
       sqlx::query(
           "SELECT vector_id, content, embedding <=> $1::vector AS distance, metadata FROM vectors WHERE tenant = $2 AND embedding <=> $1::vector < $3 ORDER BY embedding <=> $1::vector LIMIT $4 OFFSET $5"
       )
       .bind(&vectors)
       .bind(&tenant)
       .bind(distance_threshold)
       .bind(limit)
       .bind(offset)
       .fetch_all(pool)
       .await
   }
   .map_err(|e| {
       eprintln!("Database error in vector search: {}", e);
       ApiError::DatabaseError
   })?;

    let matches = rows
        .into_iter()
        .map(|row| {
            let vector_id: String = row.get("vector_id");
            let distance: f64 = row.get("distance");
            let content: Option<String> = row.get("content");
            let metadata: Option<serde_json::Value> = row.get("metadata");
            let combined_score = ((1.0 - distance) * 0.6).max(0.0).min(1.0);

            SearchResult {
                vector_id,
                content,
                metadata,
                score: Some(format!("{:.1}%", combined_score * 100.0)),
            }
        })
        .collect();

    Ok(SearchResults { matches })
}
