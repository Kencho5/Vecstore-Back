use crate::{prelude::*, structs::dashboard_struct::*};

pub async fn search_document_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Json(payload): Json<DocumentsPayload>,
) -> Result<Json<Vec<DatabaseDocument>>, DashboardError> {
    let neon_pool = state
        .neon_pools
        .get_pool_by_region(&payload.region)
        .ok_or(DashboardError::Unforseen)?;

    if payload.search_type.as_str() != "id" {
        deduct_credits(&state.pool, claims.user_id, 1, &payload.name).await?;
    }

    match payload.search_type.as_str() {
        "id" => search_by_id(payload.data, neon_pool, claims.user_id, payload.name).await,
        "text" => {
            search_by_text(
                payload.data,
                payload.db_type,
                claims.user_id,
                payload.name,
                payload.region,
                &state.bedrock_client,
                &state,
            )
            .await
        }
        "image" => {
            search_by_image(
                payload.data,
                claims.user_id,
                payload.name,
                payload.region,
                &state.bedrock_client,
                &state,
            )
            .await
        }
        _ => Err(DashboardError::Unforseen),
    }
}

async fn search_by_id(
    data: String,
    pool: &PgPool,
    user_id: i32,
    database: String,
) -> Result<Json<Vec<DatabaseDocument>>, DashboardError> {
    let tenant = format!("{}-{}", user_id, database);
    let documents = sqlx::query_as::<_, DatabaseDocument>(
        "SELECT vector_id, content, metadata FROM vectors WHERE tenant = $1 AND vector_id = $2 LIMIT 5",
    )
    .bind(&tenant)
    .bind(&data)
    .fetch_all(pool)
    .await
    .map_err(|_| 
        DashboardError::Unforseen
    )?;

    Ok(Json(documents))
}

async fn search_by_text(
    data: String,
    db_type: String,
    user_id: i32,
    database: String,
    region: String,
    bedrock_client: &BedrockClient,
    state: &AppState,
) -> Result<Json<Vec<DatabaseDocument>>, DashboardError> {
    let vectors = match db_type.as_str() {
        "text" => extract_text_features_multilingual(&bedrock_client, &data)
            .await
            .map_err(|_| DashboardError::Unforseen)?,
        "image" => extract_text_features(&bedrock_client, data.clone())
            .await
            .map_err(|_| DashboardError::Unforseen)?,
        _ => return Err(DashboardError::Unforseen),
    };

    let search_results = hybrid_search_vectors(
        state,
        &data,
        vectors,
        user_id,
        &database,
        &region,
        None,
        None,
        Some(3),
    )
    .await
    .map_err(|_| DashboardError::Unforseen)?;

    let documents = search_results
        .matches
        .into_iter()
        .map(|match_result| DatabaseDocument {
            vector_id: match_result.vector_id,
            metadata: match_result
                .metadata,
            content: match_result.content,
            score: Some(match_result.score),
        })
        .collect();

    Ok(Json(documents))
}

async fn search_by_image(
    data: String,
    user_id: i32,
    database: String,
    region: String,
    bedrock_client: &BedrockClient,
    state: &AppState,
) -> Result<Json<Vec<DatabaseDocument>>, DashboardError> {
    let image_bytes = base64::engine::general_purpose::STANDARD
        .decode(&data)
        .map_err(|_| DashboardError::Unforseen)?;

    let vectors = extract_image_features(&bedrock_client, image_bytes)
        .await
        .map_err(|_| DashboardError::Unforseen)?;

    let search_results = hybrid_search_vectors(
        state, "", vectors, user_id, &database, &region, None, None, None,
    )
    .await
    .map_err(|_| DashboardError::Unforseen)?;

    let documents = search_results
        .matches
        .into_iter()
        .map(|match_result| DatabaseDocument {
            vector_id: match_result.vector_id,
            metadata: match_result
                .metadata,
            content: match_result.content,
            score: Some(match_result.score),
        })
        .collect();

    Ok(Json(documents))
}
