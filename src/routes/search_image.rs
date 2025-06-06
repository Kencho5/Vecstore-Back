use crate::prelude::*;
use crate::structs::search_struct::*;

pub async fn search_image_handler(
    Extension(api_key): Extension<String>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<SearchResponse>, SearchImageError> {
    let total_start = Instant::now();
    let mut image_data: Option<Vec<u8>> = None;
    let mut text: Option<String> = None;
    let mut database: Option<String> = None;

    let user_id = get_user(&state.pool, api_key)
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

    let results = search_vectors(state.pinecone, vectors, user_id, &database).await?;

    let increment_task = BackgroundTask::IncrementRequest { database: database };

    if state.task_queue.send(increment_task).is_err() {
        eprintln!("Failed to send increment_task");
    }

    let total_time_ms = total_start.elapsed().as_millis() as u64;
    println!("Total search handler time: {}ms", total_time_ms);

    Ok(Json(results))
}

async fn search_vectors(
    pinecone: PineconeClient,
    vectors: Vec<f32>,
    user_id: i32,
    database: &String,
) -> Result<SearchResponse, SearchImageError> {
    let mut index = pinecone
        .index(&env::var("PINECONE_INDEX").expect("Pinecone index not found"))
        .await
        .map_err(|_| SearchImageError::Unforseen)?;

    let response: QueryResponse = index
        .query_by_value(
            vectors,
            None,
            3,
            &Namespace::from(format!("{}-{}", user_id, database)),
            None,
            None,
            Some(true),
        )
        .await
        .map_err(|_| SearchImageError::Unforseen)?;

    // Convert QueryResponse to SearchResponse
    let search_response = SearchResponse {
        matches: response
            .matches
            .into_iter()
            .map(|m| SearchMatch {
                score: m.score,
                filename: m.metadata.and_then(|metadata| {
                    metadata.fields.get("filename").and_then(|value| {
                        if let Some(Kind::StringValue(s)) = &value.kind {
                            Some(s.clone())
                        } else {
                            None
                        }
                    })
                }),
            })
            .collect(),
    };

    Ok(search_response)
}
