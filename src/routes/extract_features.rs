use crate::prelude::*;
use crate::structs::extract_features_struct::*;

pub async fn extract_features_handler(
    State(state): State<AppState>,
    Json(payload): Json<ExtractFeaturesPayload>,
) -> Result<Json<ExtractFeaturesBody>, ExtractFeaturesError> {
    let total_start = Instant::now();
    println!("Handler started");

    let image_vector = extract_image_features(&state, payload.image).await?;

    let total_time_ms = total_start.elapsed().as_millis() as u64;
    println!("Total handler time: {}ms", total_time_ms);

    Ok(Json(ExtractFeaturesBody::new(total_time_ms)))
}

async fn extract_image_features(
    state: &AppState,
    image: String,
) -> Result<Vec<f32>, ExtractFeaturesError> {
    let start_time = Instant::now();

    // Time image loading
    let load_start = Instant::now();
    let image = load_image(image, state.clip_config.image_size)
        .map_err(|_| ExtractFeaturesError::Unforseen)?;
    let load_time_ms = load_start.elapsed().as_millis();
    println!("Image loading took: {}ms", load_time_ms);

    // Time tensor operations
    let tensor_start = Instant::now();
    let batched_image = image
        .unsqueeze(0)
        .map_err(|_| ExtractFeaturesError::Unforseen)?;
    let tensor_time_ms = tensor_start.elapsed().as_millis();
    println!("Tensor batching took: {}ms", tensor_time_ms);

    // Time model inference
    let inference_start = Instant::now();
    let image_features = state
        .model
        .get_image_features(&batched_image)
        .map_err(|_| ExtractFeaturesError::Unforseen)?;
    let inference_time_ms = inference_start.elapsed().as_millis();
    println!("Model inference took: {}ms", inference_time_ms);

    // Time vector conversion
    let conversion_start = Instant::now();
    let image_vector = image_features
        .flatten_all()
        .map_err(|_| ExtractFeaturesError::Unforseen)?
        .to_vec1::<f32>()
        .map_err(|_| ExtractFeaturesError::Unforseen)?;
    let conversion_time_ms = conversion_start.elapsed().as_millis();
    println!("Vector conversion took: {}ms", conversion_time_ms);

    let total_processing_time_ms = start_time.elapsed().as_millis();
    println!(
        "Total extract_image_features took: {}ms",
        total_processing_time_ms
    );

    Ok(image_vector)
}
