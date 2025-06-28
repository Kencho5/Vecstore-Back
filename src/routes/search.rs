use crate::prelude::*;

pub async fn search_handler(
    Extension(api_key): Extension<String>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<SearchResponse>, ApiError> {
    let total_start = Instant::now();

    let mut image_data: Option<Vec<u8>> = None;
    let mut text: Option<String> = None;
    let mut database: Option<String> = None;
    let mut metadata: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| ApiError::MissingData)?
    {
        let field_name = field.name().unwrap_or("").to_string();
        match field_name.as_str() {
            "image" | "file" => {
                image_data = Some(
                    field
                        .bytes()
                        .await
                        .map_err(|_| ApiError::MissingData)?
                        .to_vec(),
                );
            }
            "text" => {
                let bytes = field.bytes().await.map_err(|_| ApiError::MissingData)?;
                text = Some(
                    std::str::from_utf8(&bytes)
                        .map_err(|_| ApiError::MissingData)?
                        .to_string(),
                );
            }
            "database" => {
                let bytes = field.bytes().await.map_err(|_| ApiError::MissingData)?;
                database = Some(
                    std::str::from_utf8(&bytes)
                        .map_err(|_| ApiError::MissingData)?
                        .to_string(),
                );
            }
            "metadata" => {
                let bytes = field.bytes().await.map_err(|_| ApiError::MissingData)?;
                metadata = Some(
                    std::str::from_utf8(&bytes)
                        .map_err(|_| ApiError::MissingData)?
                        .to_string(),
                );
            }
            _ => {} // Ignore unknown fields
        }
    }

    let database = database.ok_or(ApiError::MissingData)?;

    let validation_result = validate_user_and_increment(&state.pool, api_key, &database).await?;

    let vectors = if let Some(image_bytes) = image_data {
        extract_image_features(&state, image_bytes)
            .await
            .map_err(|_| ApiError::ModelInference)?
    } else if let Some(text_content) = text {
        extract_text_features(&state, text_content).await?
    } else {
        return Err(ApiError::MissingData); // Neither image nor text provided
    };

    let results = search_vectors_with_region(
        &state,
        vectors,
        validation_result.user_id,
        &database,
        &validation_result.region,
        metadata,
    )
    .await?;

    let total_time_ms = total_start.elapsed().as_millis() as u64;

    let logs_task = BackgroundTask::SaveUsageLogs {
        pool: state.pool,
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
