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
    let distance_threshold = 0.72;

    let use_hybrid = !query_text.trim().is_empty();

    if use_hybrid {
        hybrid_search_query(
            query_text,
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
        vector_search_query(
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

fn build_hybrid_search_query(has_metadata_filter: bool) -> &'static str {
    if has_metadata_filter {
        r#"
        WITH query_parser AS (
            SELECT to_tsquery('simple',
                COALESCE(
                    NULLIF(
                        regexp_replace(
                            regexp_replace(
                                trim(regexp_replace($4, '[^\w\s]', ' ', 'g')),
                                '\s+', ' & ', 'g'
                            ),
                            '([^ ]+)$',
                            '\1:*'
                        ),
                        ''
                    ),
                    'EMPTY_QUERY_MARKER'
                )
            ) as tsquery
        ),
        vector_candidates AS (
            SELECT vector_id,
                   ROW_NUMBER() OVER (ORDER BY embedding <=> $1::vector) as vrank
            FROM vectors
            WHERE tenant = $2 AND metadata @> $3
              AND embedding <=> $1::vector < $5
            ORDER BY embedding <=> $1::vector
            LIMIT $6
        ),
        text_candidates AS (
            SELECT vector_id,
                   ROW_NUMBER() OVER (
                       ORDER BY ts_rank_cd(search_vector, query_parser.tsquery) DESC
                   ) as trank
            FROM vectors
            CROSS JOIN query_parser
            WHERE tenant = $2 AND metadata @> $3
            AND search_vector @@ query_parser.tsquery
            LIMIT $6
        ),
        rrf_scores AS (
            SELECT
                COALESCE(v.vector_id, t.vector_id) as vector_id,
                (COALESCE(1.0 / (60.0 + v.vrank), 0.0) +
                 COALESCE(1.0 / (60.0 + t.trank), 0.0))::FLOAT8 as score
            FROM vector_candidates v
            FULL OUTER JOIN text_candidates t ON v.vector_id = t.vector_id
        ),
        final_ids AS (
            SELECT vector_id, score
            FROM rrf_scores
            ORDER BY score DESC
            LIMIT $7 OFFSET $8
        )
        SELECT v.vector_id, v.content, v.metadata, f.score
        FROM final_ids f
        JOIN vectors v ON v.vector_id = f.vector_id
        ORDER BY f.score DESC
        "#
    } else {
        r#"
        WITH query_parser AS (
            SELECT to_tsquery('simple',
                COALESCE(
                    NULLIF(
                        regexp_replace(
                            regexp_replace(
                                trim(regexp_replace($3, '[^\w\s]', ' ', 'g')),
                                '\s+', ' & ', 'g'
                            ),
                            '([^ ]+)$',
                            '\1:*'
                        ),
                        ''
                    ),
                    'EMPTY_QUERY_MARKER'
                )
            ) as tsquery
        ),
        vector_candidates AS (
            SELECT vector_id,
                   ROW_NUMBER() OVER (ORDER BY embedding <=> $1::vector) as vrank
            FROM vectors
            WHERE tenant = $2
              AND embedding <=> $1::vector < $4
            ORDER BY embedding <=> $1::vector
            LIMIT $5
        ),
        text_candidates AS (
            SELECT vector_id,
                   ROW_NUMBER() OVER (
                       ORDER BY ts_rank_cd(search_vector, query_parser.tsquery) DESC
                   ) as trank
            FROM vectors
            CROSS JOIN query_parser
            WHERE tenant = $2
            AND search_vector @@ query_parser.tsquery
            LIMIT $5
        ),
        rrf_scores AS (
            SELECT
                COALESCE(v.vector_id, t.vector_id) as vector_id,
                (COALESCE(1.0 / (60.0 + v.vrank), 0.0) +
                 COALESCE(1.0 / (60.0 + t.trank), 0.0))::FLOAT8 as score
            FROM vector_candidates v
            FULL OUTER JOIN text_candidates t ON v.vector_id = t.vector_id
        ),
        final_ids AS (
            SELECT vector_id, score
            FROM rrf_scores
            ORDER BY score DESC
            LIMIT $6 OFFSET $7
        )
        SELECT v.vector_id, v.content, v.metadata, f.score
        FROM final_ids f
        JOIN vectors v ON v.vector_id = f.vector_id
        ORDER BY f.score DESC
        "#
    }
}

async fn hybrid_search_query(
    query_text: &str,
    vectors: &[f32],
    tenant: &str,
    pool: &PgPool,
    metadata_filter: Option<serde_json::Value>,
    distance_threshold: f64,
    oversample_limit: i64,
    limit: i64,
    offset: i64,
) -> Result<SearchResults, ApiError> {
    let sql = build_hybrid_search_query(metadata_filter.is_some());

    let query = sqlx::query(sql);

    let rows = if let Some(metadata_json) = metadata_filter {
        query
            .bind(vectors)
            .bind(tenant)
            .bind(metadata_json)
            .bind(query_text)
            .bind(distance_threshold)
            .bind(oversample_limit)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
    } else {
        query
            .bind(vectors)
            .bind(tenant)
            .bind(query_text)
            .bind(distance_threshold)
            .bind(oversample_limit)
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
            let score: f64 = row.get("score");
            let content: Option<String> = row.get("content");
            let metadata: Option<serde_json::Value> = row.get("metadata");

            let max_rrf = 2.0 / 61.0;
            let normalized = (score / max_rrf * 100.0).min(100.0);

            SearchResult {
                vector_id,
                content,
                metadata,
                score: Some(format!("{:.1}%", normalized)),
            }
        })
        .collect();

    Ok(SearchResults { matches })
}

fn build_vector_search_query(has_metadata_filter: bool) -> &'static str {
    if has_metadata_filter {
        r#"
        WITH candidates AS (
            SELECT vector_id, embedding <=> $1::vector AS distance
            FROM vectors
            WHERE tenant = $2 AND metadata @> $3
            ORDER BY embedding <=> $1::vector
            LIMIT $4
        ),
        final_ids AS (
            SELECT vector_id, distance
            FROM candidates
            WHERE distance < $5
            ORDER BY distance
            LIMIT $6 OFFSET $7
        )
        SELECT v.vector_id, v.content, v.metadata, f.distance
        FROM final_ids f
        JOIN vectors v ON v.vector_id = f.vector_id
        ORDER BY f.distance
        "#
    } else {
        r#"
        WITH candidates AS (
            SELECT vector_id, embedding <=> $1::vector AS distance
            FROM vectors
            WHERE tenant = $2
            ORDER BY embedding <=> $1::vector
            LIMIT $3
        ),
        final_ids AS (
            SELECT vector_id, distance
            FROM candidates
            WHERE distance < $4
            ORDER BY distance
            LIMIT $5 OFFSET $6
        )
        SELECT v.vector_id, v.content, v.metadata, f.distance
        FROM final_ids f
        JOIN vectors v ON v.vector_id = f.vector_id
        ORDER BY f.distance
        "#
    }
}

async fn vector_search_query(
    vectors: &[f32],
    tenant: &str,
    pool: &PgPool,
    metadata_filter: Option<serde_json::Value>,
    distance_threshold: f64,
    oversample_limit: i64,
    limit: i64,
    offset: i64,
) -> Result<SearchResults, ApiError> {
    let sql = build_vector_search_query(metadata_filter.is_some());
    let query = sqlx::query(sql);

    let rows = if let Some(metadata_json) = metadata_filter {
        query
            .bind(vectors)
            .bind(tenant)
            .bind(metadata_json)
            .bind(oversample_limit)
            .bind(distance_threshold)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
    } else {
        query
            .bind(vectors)
            .bind(tenant)
            .bind(oversample_limit)
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
            let score = ((1.0 - distance) * 100.0).max(0.0).min(100.0);

            SearchResult {
                vector_id,
                content,
                metadata,
                score: Some(format!("{:.1}%", score)),
            }
        })
        .collect();

    Ok(SearchResults { matches })
}
