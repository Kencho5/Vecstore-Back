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

    let oversample_limit = ((limit + offset) * 8).max(150).min(800);
    let distance_threshold = 0.7;

    let use_hybrid = !query_text.trim().is_empty();

    if use_hybrid {
        let tsquery = build_tsquery(query_text);
        hybrid_search(
            &tsquery,
            &vectors,
            &tenant,
            pool,
            metadata_filter,
            distance_threshold,
            oversample_limit,
            limit,
            offset,
        )
        .await
    } else {
        vector_search(
            &vectors,
            &tenant,
            pool,
            metadata_filter,
            distance_threshold,
            oversample_limit,
            limit,
            offset,
        )
        .await
    }
}

fn build_tsquery(text: &str) -> String {
    let words: Vec<&str> = text
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .collect();

    if words.is_empty() {
        return "EMPTY_QUERY_MARKER".to_string();
    }

    let last = words.len() - 1;
    words
        .iter()
        .enumerate()
        .map(|(i, w)| {
            if i == last {
                format!("{}:*", w)
            } else {
                w.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" & ")
}

const HYBRID_SQL: &str = r#"
WITH vector_candidates AS (
    SELECT id,
           ROW_NUMBER() OVER (ORDER BY dist) AS vrank
    FROM (
        SELECT id, embedding <=> $1::vector AS dist
        FROM vectors
        WHERE tenant = $2
        ORDER BY embedding <=> $1::vector
        LIMIT $5
    ) v_sub
    WHERE dist < $4
),
text_candidates AS (
    SELECT id,
           ROW_NUMBER() OVER (
               ORDER BY ts_rank_cd(search_vector, to_tsquery('simple', $3)) DESC
           ) AS trank
    FROM vectors
    WHERE tenant = $2
      AND search_vector @@ to_tsquery('simple', $3)
    LIMIT $5
),
rrf_scores AS (
    SELECT
        COALESCE(v.id, t.id) AS id,
        (COALESCE(1.0 / (60.0 + v.vrank), 0.0) +
         COALESCE(1.0 / (60.0 + t.trank), 0.0))::FLOAT8 AS score
    FROM vector_candidates v
    FULL OUTER JOIN text_candidates t ON v.id = t.id
),
final_ids AS (
    SELECT id, score
    FROM rrf_scores
    ORDER BY score DESC
    LIMIT $6 OFFSET $7
)
SELECT v.vector_id, v.content, v.metadata, f.score
FROM final_ids f
JOIN vectors v ON v.id = f.id
ORDER BY f.score DESC
"#;

const HYBRID_SQL_META: &str = r#"
WITH vector_candidates AS (
    SELECT id,
           ROW_NUMBER() OVER (ORDER BY dist) AS vrank
    FROM (
        SELECT id, embedding <=> $1::vector AS dist
        FROM vectors
        WHERE tenant = $2 AND metadata @> $3
        ORDER BY embedding <=> $1::vector
        LIMIT $6
    ) v_sub
    WHERE dist < $5
),
text_candidates AS (
    SELECT id,
           ROW_NUMBER() OVER (
               ORDER BY ts_rank_cd(search_vector, to_tsquery('simple', $4)) DESC
           ) AS trank
    FROM vectors
    WHERE tenant = $2 AND metadata @> $3
      AND search_vector @@ to_tsquery('simple', $4)
    LIMIT $6
),
rrf_scores AS (
    SELECT
        COALESCE(v.id, t.id) AS id,
        (COALESCE(1.0 / (60.0 + v.vrank), 0.0) +
         COALESCE(1.0 / (60.0 + t.trank), 0.0))::FLOAT8 AS score
    FROM vector_candidates v
    FULL OUTER JOIN text_candidates t ON v.id = t.id
),
final_ids AS (
    SELECT id, score
    FROM rrf_scores
    ORDER BY score DESC
    LIMIT $7 OFFSET $8
)
SELECT v.vector_id, v.content, v.metadata, f.score
FROM final_ids f
JOIN vectors v ON v.id = f.id
ORDER BY f.score DESC
"#;

async fn hybrid_search(
    tsquery: &str,
    vectors: &[f32],
    tenant: &str,
    pool: &PgPool,
    metadata_filter: Option<serde_json::Value>,
    distance_threshold: f64,
    oversample_limit: i64,
    limit: i64,
    offset: i64,
) -> Result<SearchResults, ApiError> {
    let rows = if let Some(meta) = metadata_filter {
        sqlx::query(HYBRID_SQL_META)
            .bind(vectors) // $1
            .bind(tenant) // $2
            .bind(meta) // $3
            .bind(tsquery) // $4
            .bind(distance_threshold) // $5
            .bind(oversample_limit) // $6
            .bind(limit) // $7
            .bind(offset) // $8
            .fetch_all(pool)
            .await
    } else {
        sqlx::query(HYBRID_SQL)
            .bind(vectors) // $1
            .bind(tenant) // $2
            .bind(tsquery) // $3
            .bind(distance_threshold) // $4
            .bind(oversample_limit) // $5
            .bind(limit) // $6
            .bind(offset) // $7
            .fetch_all(pool)
            .await
    }
    .map_err(|e| {
        eprintln!("Database error in hybrid search: {}", e);
        ApiError::DatabaseError
    })?;

    const MAX_RRF: f64 = 2.0 / 61.0;

    let matches = rows
        .into_iter()
        .map(|row| {
            let score: f64 = row.get("score");
            SearchResult {
                vector_id: row.get("vector_id"),
                content: row.get("content"),
                metadata: row.get("metadata"),
                score: Some(format!("{:.1}%", (score / MAX_RRF * 100.0).min(100.0))),
            }
        })
        .collect();

    Ok(SearchResults { matches })
}

const VECTOR_SQL: &str = r#"
SELECT vector_id, content, metadata, distance
FROM (
    SELECT vector_id, content, metadata, embedding <=> $1::vector AS distance
    FROM vectors
    WHERE tenant = $2
    ORDER BY embedding <=> $1::vector
    LIMIT $3
) sub
WHERE distance < $4
ORDER BY distance
LIMIT $5 OFFSET $6
"#;

const VECTOR_SQL_META: &str = r#"
SELECT vector_id, content, metadata, distance
FROM (
    SELECT vector_id, content, metadata, embedding <=> $1::vector AS distance
    FROM vectors
    WHERE tenant = $2 AND metadata @> $3
    ORDER BY embedding <=> $1::vector
    LIMIT $4
) sub
WHERE distance < $5
ORDER BY distance
LIMIT $6 OFFSET $7
"#;

async fn vector_search(
    vectors: &[f32],
    tenant: &str,
    pool: &PgPool,
    metadata_filter: Option<serde_json::Value>,
    distance_threshold: f64,
    oversample_limit: i64,
    limit: i64,
    offset: i64,
) -> Result<SearchResults, ApiError> {
    let rows = if let Some(meta) = metadata_filter {
        sqlx::query(VECTOR_SQL_META)
            .bind(vectors) // $1
            .bind(tenant) // $2
            .bind(meta) // $3
            .bind(oversample_limit) // $4
            .bind(distance_threshold) // $5
            .bind(limit) // $6
            .bind(offset) // $7
            .fetch_all(pool)
            .await
    } else {
        sqlx::query(VECTOR_SQL)
            .bind(vectors) // $1
            .bind(tenant) // $2
            .bind(oversample_limit) // $3
            .bind(distance_threshold) // $4
            .bind(limit) // $5
            .bind(offset) // $6
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
            let distance: f64 = row.get("distance");
            SearchResult {
                vector_id: row.get("vector_id"),
                content: row.get("content"),
                metadata: row.get("metadata"),
                score: Some(format!(
                    "{:.1}%",
                    ((1.0 - distance) * 100.0).clamp(0.0, 100.0)
                )),
            }
        })
        .collect();

    Ok(SearchResults { matches })
}
