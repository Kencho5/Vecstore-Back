use crate::prelude::*;
use aws_sdk_bedrockruntime::primitives::Blob;
use base64::{engine::general_purpose, Engine as _};
use serde_json::{json, Value};

pub async fn extract_image_features(
    bedrock_client: &BedrockClient,
    image_data: Vec<u8>,
) -> Result<Vec<f32>, ApiError> {
    // Resize image before encoding
    let resized_image_data = resize_image(image_data)?;

    let base64_image = general_purpose::STANDARD.encode(&resized_image_data);
    let body = json!({
        "inputImage": base64_image,
        "embeddingConfig": {
            "outputEmbeddingLength": 384
        }
    });

    let body_bytes = serde_json::to_vec(&body).map_err(|_| ApiError::ImageProcessing)?;
    let response = bedrock_client
        .invoke_model()
        .model_id("amazon.titan-embed-image-v1")
        .content_type("application/json")
        .body(Blob::new(body_bytes))
        .send()
        .await
        .map_err(|e| {
            eprintln!("Bedrock error: {:?}", e);
            ApiError::Unforseen
        })?;

    let response_body = response.body().as_ref();
    let response_json: Value =
        serde_json::from_slice(response_body).map_err(|_| ApiError::ModelInference)?;
    let embedding = response_json["embedding"]
        .as_array()
        .ok_or(ApiError::ModelInference)?
        .iter()
        .map(|v| v.as_f64().unwrap_or(0.0) as f32)
        .collect::<Vec<f32>>();

    Ok(embedding)
}

pub async fn extract_text_features(
    bedrock_client: &BedrockClient,
    text: String,
) -> Result<Vec<f32>, ApiError> {
    let body = json!({
        "inputText": text,
        "embeddingConfig": {
            "outputEmbeddingLength": 384
        }
    });

    let body_bytes = serde_json::to_vec(&body).map_err(|_| ApiError::Unforseen)?;

    let response = bedrock_client
        .invoke_model()
        .model_id("amazon.titan-embed-image-v1")
        .content_type("application/json")
        .body(Blob::new(body_bytes))
        .send()
        .await
        .map_err(|e| {
            eprintln!("Bedrock text embedding error: {:?}", e);
            ApiError::Unforseen
        })?;

    let response_body = response.body().as_ref();
    let response_json: Value =
        serde_json::from_slice(response_body).map_err(|_| ApiError::ModelInference)?;

    let embedding = response_json["embedding"]
        .as_array()
        .ok_or(ApiError::Unforseen)?
        .iter()
        .map(|v| v.as_f64().unwrap_or(0.0) as f32)
        .collect::<Vec<f32>>();

    Ok(embedding)
}
