use crate::prelude::*;
use crate::structs::insert_struct::*;

pub async fn insert_image_handler(
    State(state): State<AppState>,
    Json(payload): Json<InsertImagePayload>,
) -> Result<Json<InsertImageBody>, InsertImageError> {
    let total_start = Instant::now();
    println!("Handler started");

    let image_vectors = extract_image_features(&state, payload.image).await?;
    insert_vectors(state.http_client, image_vectors, payload.filename).await?;

    let total_time_ms = total_start.elapsed().as_millis() as u64;
    println!("Total handler time: {}ms", total_time_ms);
    Ok(Json(InsertImageBody::new(total_time_ms)))
}

async fn extract_image_features(
    state: &AppState,
    image: String,
) -> Result<Vec<f32>, InsertImageError> {
    let start_time = Instant::now();

    let image = load_image(image, state.clip_config.image_size)
        .map_err(|_| InsertImageError::ImageProcessing)?;

    let batched_image = image
        .unsqueeze(0)
        .map_err(|_| InsertImageError::ImageProcessing)?;

    let image_features = state
        .model
        .get_image_features(&batched_image)
        .map_err(|_| InsertImageError::ModelInference)?;

    let image_vector = image_features
        .flatten_all()
        .map_err(|_| InsertImageError::ImageProcessing)?
        .to_vec1::<f32>()
        .map_err(|_| InsertImageError::ImageProcessing)?;

    let total_processing_time_ms = start_time.elapsed().as_millis();
    println!(
        "Total extract_image_features took: {}ms",
        total_processing_time_ms
    );

    Ok(image_vector)
}

async fn insert_vectors(
    client: Client,
    vectors: Vec<f32>,
    filename: String,
) -> Result<(), InsertImageError> {
    let body = json!({
        "points": [
            {
                "id": Uuid::new_v4().to_string(),
                "vector": vectors,
                "payload": { "filename": filename }
            },
        ]
    });

    let response = client
        .put("https://db.vecstore.app/collections/images/points")
        .json(&body)
        .send()
        .await
        .map_err(|_| InsertImageError::DatabaseConnection)?;

    if !response.status().is_success() {
        return Err(InsertImageError::DatabaseInsert);
    }

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|_| InsertImageError::DatabaseResponse)?;

    println!("{}", json);
    Ok(())
}
