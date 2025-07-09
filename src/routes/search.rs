use crate::prelude::*;

pub async fn search_handler(
    Extension(api_key): Extension<String>,
    State(state): State<AppState>,
    Json(payload): Json<SearchPayload>,
) -> Result<Json<SearchResponse>, ApiError> {
    let total_start = Instant::now();
    let validation_result =
        validate_user_and_increment(&state.pool, api_key, &payload.database).await?;

    let vectors = if let Some(base64_image) = &payload.image {
        let image_bytes = base64::engine::general_purpose::STANDARD
            .decode(base64_image)
            .map_err(|_| ApiError::ImageProcessing)?;
        extract_image_features(&state.bedrock_client, image_bytes)
            .await
            .map_err(|_| ApiError::ModelInference)?
    } else if validation_result.db_type == "image" {
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
        validation_result.user_id,
        &payload.database,
        &validation_result.region,
        payload.metadata,
        payload.page,
        payload.limit,
    )
    .await?;

    let total_time_ms = total_start.elapsed().as_millis() as u64;
    let logs_task = BackgroundTask::SaveUsageLogs {
        user_id: validation_result.user_id,
    };
    if state.task_queue.send(logs_task).is_err() {
        eprintln!("Failed to send logs_task");
    }
    Ok(Json(SearchResponse {
        results: results.matches,
        time: format!("{}ms", total_time_ms),
        credits_left: validation_result.credits_left,
    }))
}
