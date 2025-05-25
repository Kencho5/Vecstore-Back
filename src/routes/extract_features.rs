use crate::prelude::*;
use crate::structs::extract_features_struct::*;

pub async fn extract_features_handler(
    State(state): State<AppState>,
    Json(payload): Json<ExtractFeaturesPayload>,
) -> Result<Json<ExtractFeaturesBody>, ExtractFeaturesError> {
    let start_time = Instant::now();
    let image_vector = extract_image_features(&state, payload.image).await?;
    let processing_time_ms = start_time.elapsed().as_millis() as u64;

    println!("Image processing took: {}ms", processing_time_ms);
    println!("Image vector: {:?}", image_vector);

    Ok(Json(ExtractFeaturesBody::new(processing_time_ms)))
}

async fn extract_image_features(
    state: &AppState,
    image: String,
) -> Result<Vec<f32>, ExtractFeaturesError> {
    let image = load_image(image, state.clip_config.image_size)
        .map_err(|_| ExtractFeaturesError::Unforseen)?;

    let batched_image = image
        .unsqueeze(0)
        .map_err(|_| ExtractFeaturesError::Unforseen)?;

    let image_features = state
        .model
        .get_image_features(&batched_image)
        .map_err(|_| ExtractFeaturesError::Unforseen)?;

    let image_vector = image_features
        .flatten_all()
        .map_err(|_| ExtractFeaturesError::Unforseen)?
        .to_vec1::<f32>()
        .map_err(|_| ExtractFeaturesError::Unforseen)?;

    Ok(image_vector)
}
