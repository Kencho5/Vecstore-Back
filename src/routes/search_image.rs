use crate::prelude::*;
use crate::structs::search_struct::*;

pub async fn search_image_handler(
    Extension(api_key): Extension<String>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<SearchResponse>, SearchImageError> {
    let mut image_data: Option<Vec<u8>> = None;
    let mut text: Option<String> = None;
    let mut database: Option<String> = None;

    let user_id = get_user_key(&state.pool, api_key, "Image Search".to_string())
        .await
        .map_err(|_| SearchImageError::InvalidApiKey)?;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| SearchImageError::MissingData)?
    {
        let field_name = field.name().unwrap_or("").to_string();
        match field_name.as_str() {
            "image" | "file" => {
                image_data = Some(
                    field
                        .bytes()
                        .await
                        .map_err(|_| SearchImageError::MissingData)?
                        .to_vec(),
                );
            }
            "text" => {
                let bytes = field
                    .bytes()
                    .await
                    .map_err(|_| SearchImageError::MissingData)?;
                text = Some(
                    String::from_utf8(bytes.to_vec()).map_err(|_| SearchImageError::MissingData)?,
                );
            }
            "database" => {
                let bytes = field
                    .bytes()
                    .await
                    .map_err(|_| SearchImageError::MissingData)?;
                database = Some(
                    String::from_utf8(bytes.to_vec()).map_err(|_| SearchImageError::MissingData)?,
                );
            }
            _ => {} // Ignore unknown fields
        }
    }

    let database = database.ok_or(SearchImageError::MissingData)?;

    let vectors = if let Some(image_bytes) = image_data {
        extract_image_features(&state, image_bytes)
            .await
            .map_err(|_| SearchImageError::ModelInference)?
    } else if let Some(text_content) = text {
        extract_text_features(&state, text_content).await?
    } else {
        return Err(SearchImageError::MissingData); // Neither image nor text provided
    };

    let indexes = state.pinecone_indexes.lock().await;
    let index = indexes.image_us_east.clone();
    let results = search_vectors(index, vectors, user_id, &database).await?;

    let increment_task = BackgroundTask::IncrementRequest { database, user_id };

    if state.task_queue.send(increment_task).is_err() {
        eprintln!("Failed to send increment_task");
    }

    Ok(Json(results))
}
