use crate::prelude::*;
use crate::structs::{insert_struct::*, search_struct::*};

pub async fn extract_image_features(
    state: &AppState,
    image: String,
) -> Result<Vec<f32>, InsertImageError> {
    let image = load_image::load_image(image, state.clip_config.image_size)
        .map_err(|_| InsertImageError::ImageProcessing)?;

    let image_features = state
        .clip_model
        .get_image_features(&image)
        .map_err(|_| InsertImageError::ModelInference)?;

    let image_vector = image_features
        .flatten_all()
        .map_err(|_| InsertImageError::ImageProcessing)?
        .to_vec1::<f32>()
        .map_err(|_| InsertImageError::ImageProcessing)?;

    Ok(image_vector)
}

pub async fn extract_text_features(
    state: &AppState,
    text: String,
) -> Result<Vec<f32>, SearchImageError> {
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

    Ok(text_vector)
}
