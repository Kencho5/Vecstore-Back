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

    // Validate user and increment request count in one call
    let validation_result = validate_user_and_increment(
        &state.pool,
        api_key,
        database.clone(),
        "Image Search".to_string(),
    )
    .await
    .map_err(|_| SearchImageError::InvalidApiKey)?;

    let vectors = if let Some(image_bytes) = image_data {
        extract_image_features(&state, image_bytes)
            .await
            .map_err(|_| SearchImageError::ModelInference)?
    } else if let Some(text_content) = text {
        extract_text_features(&state, text_content).await?
    } else {
        return Err(SearchImageError::MissingData); // Neither image nor text provided
    };

    let results = search_vectors_with_region(&state, vectors, validation_result.user_id, &database, &validation_result.region).await?;

    Ok(Json(results))
}
