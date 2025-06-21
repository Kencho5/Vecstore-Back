use crate::prelude::*;
use crate::structs::insert_struct::*;

pub async fn insert_image_handler(
    Extension(api_key): Extension<String>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<InsertImageBody>, InsertImageError> {
    let total_start = Instant::now();

    let user_id = get_user_key(&state.pool, api_key, "Image Search".to_string()).await?;

    let mut image_data: Option<Vec<u8>> = None;
    let mut filename: Option<String> = None;
    let mut database: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| InsertImageError::MissingData)?
    {
        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "image" | "file" => {
                image_data = Some(
                    field
                        .bytes()
                        .await
                        .map_err(|_| InsertImageError::MissingData)?
                        .to_vec(),
                );
            }
            "filename" => {
                let bytes = field
                    .bytes()
                    .await
                    .map_err(|_| InsertImageError::MissingData)?;
                filename = Some(
                    String::from_utf8(bytes.to_vec()).map_err(|_| InsertImageError::MissingData)?,
                );
            }
            "database" => {
                let bytes = field
                    .bytes()
                    .await
                    .map_err(|_| InsertImageError::MissingData)?;
                database = Some(
                    String::from_utf8(bytes.to_vec()).map_err(|_| InsertImageError::MissingData)?,
                );
            }
            _ => {} // Ignore unknown fields
        }
    }

    let image_data = image_data.ok_or(InsertImageError::MissingData)?;
    let filename = filename.ok_or(InsertImageError::MissingData)?;
    let database = database.ok_or(InsertImageError::MissingData)?;

    let image_vectors = extract_image_features(&state, image_data).await?;

    let insert_task = BackgroundTask::InsertVectors {
        user_id,
        vectors: image_vectors,
        filename: filename.clone(),
        database: database.clone(),
    };
    let increment_task = BackgroundTask::IncrementRequest { database, user_id };

    if state.task_queue.send(insert_task).is_err() {
        eprintln!("Failed to send insert_task");
    }

    if state.task_queue.send(increment_task).is_err() {
        eprintln!("Failed to send increment_task");
    }

    let total_time_ms = total_start.elapsed().as_millis() as u64;

    Ok(Json(InsertImageBody::new(format!("{}ms", total_time_ms))))
}
