use crate::prelude::*;
use crate::structs::search_struct::*;

pub async fn search_image_handler(
    Extension(api_key): Extension<String>,
    State(state): State<AppState>,
    Json(payload): Json<SearchImagePayload>,
) -> Result<Json<SearchResponse>, SearchImageError> {
    payload
        .validate()
        .map_err(|_| SearchImageError::MissingData)?;

    let total_start = Instant::now();

    let user_id = get_user(&state.pool, api_key)
        .await
        .map_err(|_| SearchImageError::InvalidApiKey)?;

    let text_vectors = extract_text_features(&state, payload.text).await?;
    let results = search_vectors(state.pinecone, text_vectors, user_id, &payload.database).await?;

    let increment_task = BackgroundTask::IncrementRequest {
        database: payload.database,
    };
    if state.task_queue.send(increment_task).is_err() {
        eprintln!("Failed to send increment_task");
    }

    let total_time_ms = total_start.elapsed().as_millis() as u64;
    println!("Total text handler time: {}ms", total_time_ms);

    Ok(Json(results))
}

async fn extract_text_features(
    state: &AppState,
    text: String,
) -> Result<Vec<f32>, SearchImageError> {
    let start_time = Instant::now();

    let tokenizer = &state.tokenizer;

    let encoding = tokenizer
        .encode(text, true)
        .map_err(|_| SearchImageError::ModelInference)?;
    let tokens = encoding.get_ids().to_vec();

    let input_ids =
        Tensor::new(vec![tokens], &Device::Cpu).map_err(|_| SearchImageError::ModelInference)?;

    let text_features = state
        .clip_model
        .get_text_features(&input_ids)
        .map_err(|_| SearchImageError::ModelInference)?;

    let text_vector = text_features
        .flatten_all()
        .map_err(|_| SearchImageError::ModelInference)?
        .to_vec1::<f32>()
        .map_err(|_| SearchImageError::ModelInference)?;

    let total_processing_time_ms = start_time.elapsed().as_millis();
    println!(
        "Total extract_text_features took: {}ms",
        total_processing_time_ms
    );

    Ok(text_vector)
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
