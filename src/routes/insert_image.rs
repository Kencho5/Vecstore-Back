use crate::prelude::*;
use crate::structs::insert_struct::*;

pub async fn insert_image_handler(
    Extension(api_key): Extension<String>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<InsertImageBody>, InsertError> {
    let total_start = Instant::now();

    let mut image_data: Option<Vec<u8>> = None;
    let mut filename: Option<String> = None;
    let mut database: Option<String> = None;
    let mut metadata: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| InsertError::MissingData)?
    {
        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "image" | "file" => {
                image_data = Some(
                    field
                        .bytes()
                        .await
                        .map_err(|_| InsertError::MissingData)?
                        .to_vec(),
                );
            }
            "filename" => {
                let bytes = field.bytes().await.map_err(|_| InsertError::MissingData)?;
                filename =
                    Some(String::from_utf8(bytes.to_vec()).map_err(|_| InsertError::MissingData)?);
            }
            "database" => {
                let bytes = field.bytes().await.map_err(|_| InsertError::MissingData)?;
                database =
                    Some(String::from_utf8(bytes.to_vec()).map_err(|_| InsertError::MissingData)?);
            }
            "metadata" => {
                let bytes = field.bytes().await.map_err(|_| InsertError::MissingData)?;
                metadata =
                    Some(String::from_utf8(bytes.to_vec()).map_err(|_| InsertError::MissingData)?);
            }

            _ => {} // Ignore unknown fields
        }
    }

    let image_data = image_data.ok_or(InsertError::MissingData)?;
    let filename = filename.ok_or(InsertError::MissingData)?;
    let database = database.ok_or(InsertError::MissingData)?;

    let validation_result = validate_user_and_increment(
        &state.pool,
        api_key,
        database.clone(),
        "Image Search".to_string(),
    )
    .await?;

    let image_vectors = extract_image_features(&state, image_data).await?;

    let insert_task = BackgroundTask::InsertVectors {
        user_id: validation_result.user_id,
        vectors: image_vectors,
        filename: Some(filename),
        metadata,
        database: database.clone(),
        region: validation_result.region,
    };

    if state.task_queue.send(insert_task).is_err() {
        eprintln!("Failed to send insert_task");
    }

    let total_time_ms = total_start.elapsed().as_millis() as u64;

    Ok(Json(InsertImageBody::new(format!("{}ms", total_time_ms))))
}
