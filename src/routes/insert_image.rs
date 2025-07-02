use crate::prelude::*;

pub async fn insert_image_handler(
    Extension(api_key): Extension<String>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<InsertImageBody>, ApiError> {
    let total_start = Instant::now();

    let mut image_data: Option<Vec<u8>> = None;
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

    let image_data = image_data.ok_or(ApiError::MissingData)?;
    let database = database.ok_or(ApiError::MissingData)?;

    let validation_result = validate_user_and_increment(&state.pool, api_key, &database).await?;

    let insert_task = BackgroundTask::InsertImageVectors {
        user_id: validation_result.user_id,
        image_data,
        metadata,
        database,
        region: validation_result.region,
    };

    let logs_task = BackgroundTask::SaveUsageLogs {
        user_id: validation_result.user_id,
    };

    if state.task_queue.send(insert_task).is_err() {
        eprintln!("Failed to send insert_task");
    }

    if state.task_queue.send(logs_task).is_err() {
        eprintln!("Failed to send logs_task");
    }

    let total_time_ms = total_start.elapsed().as_millis() as u64;

    Ok(Json(InsertImageBody {
        time: format!("{}ms", total_time_ms),
        credits_left: validation_result.credits_left,
    }))
}
