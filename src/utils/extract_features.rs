use crate::prelude::*;

pub async fn extract_image_features(
    state: &AppState,
    image_data: Vec<u8>,
) -> Result<Vec<f32>, ApiError> {
    let image = load_image::load_image(image_data, state.clip_config.image_size)
        .map_err(|_| ApiError::ImageProcessing)?;

    let image_features = state
        .clip_model
        .get_image_features(&image)
        .map_err(|_| ApiError::ModelInference)?;

    let image_vector = image_features
        .flatten_all()
        .map_err(|_| ApiError::ImageProcessing)?
        .to_vec1::<f32>()
        .map_err(|_| ApiError::ImageProcessing)?;
    Ok(image_vector)
}

pub async fn extract_text_features(state: &AppState, text: String) -> Result<Vec<f32>, ApiError> {
    let tokenizer = &state.tokenizer;

    let encoding = tokenizer
        .encode(text, true)
        .map_err(|_| ApiError::ModelInference)?;
    let tokens = encoding.get_ids().to_vec();

    let input_ids =
        Tensor::new(vec![tokens], &Device::Cpu).map_err(|_| ApiError::ModelInference)?;

    let text_features = state
        .clip_model
        .get_text_features(&input_ids)
        .map_err(|_| ApiError::ModelInference)?;

    let text_vector = text_features
        .flatten_all()
        .map_err(|_| ApiError::ModelInference)?
        .to_vec1::<f32>()
        .map_err(|_| ApiError::ModelInference)?;

    Ok(text_vector)
}
