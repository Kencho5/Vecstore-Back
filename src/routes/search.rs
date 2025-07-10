use crate::prelude::*;

pub async fn search_handler(
    Extension(api_key): Extension<String>,
    State(state): State<AppState>,
    Json(payload): Json<SearchPayload>,
) -> Result<Json<SearchResponse>, ApiError> {
    let total_start = Instant::now();

    let cached_user = get_cached_user(&state, api_key, &payload.database).await?;

    let vectors = if let Some(base64_image) = &payload.image {
        let image_bytes = base64::engine::general_purpose::STANDARD
            .decode(base64_image)
            .map_err(|_| ApiError::ImageProcessing)?;
        extract_image_features(&state.bedrock_client, image_bytes)
            .await
            .map_err(|_| ApiError::ModelInference)?
    } else if cached_user.db_type == "image" {
        let text_content = payload.text.as_ref().ok_or(ApiError::MissingData)?;
        extract_text_features(&state.bedrock_client, text_content.clone()).await?
    } else if let Some(text_content) = &payload.text {
        extract_text_features_multilingual(&state.bedrock_client, text_content).await?
    } else {
        return Err(ApiError::MissingData);
    };

    let results = hybrid_search_vectors(
        &state,
        &payload.text.as_deref().unwrap_or_default(),
        vectors,
        cached_user.user_id,
        &payload.database,
        &cached_user.region,
        payload.metadata,
        payload.page,
        payload.limit,
    )
    .await?;

    let total_time_ms = total_start.elapsed().as_millis() as u64;
    
    let user_action_task = BackgroundTask::ProcessUserAction {
        user_id: cached_user.user_id,
        database: payload.database.clone(),
    };

    if state.task_queue.send(user_action_task).is_err() {
        eprintln!("Failed to send user action task");
    }
    Ok(Json(SearchResponse {
        results: results.matches,
        time: format!("{}ms", total_time_ms),
    }))
}
