use crate::prelude::*;
use aws_sdk_bedrockruntime::primitives::Blob;
use base64::Engine;
use serde_json::{json, Value};

const EMBEDDING_DIMENSIONS: u32 = 1024;

async fn invoke_bedrock_model(
    bedrock_client: &BedrockClient,
    model_id: &str,
    body: Value,
) -> Result<Vec<f32>, ApiError> {
    let body_bytes = serde_json::to_vec(&body).map_err(|_| ApiError::ModelInference)?;
    let response = bedrock_client
        .invoke_model()
        .model_id(model_id)
        .content_type("application/json")
        .body(Blob::new(body_bytes))
        .send()
        .await
        .map_err(|_| ApiError::ModelInference)?;

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

pub async fn extract_image_features(
    bedrock_client: &BedrockClient,
    base64_image: &str,
) -> Result<Vec<f32>, ApiError> {
    const MAX_BASE64_SIZE: usize = 14 * 1024 * 1024; // ~10 MB decoded
    if base64_image.len() > MAX_BASE64_SIZE {
        return Err(ApiError::ImageProcessing);
    }

    let body = json!({
        "inputImage": base64_image,
        "embeddingConfig": {
            "outputEmbeddingLength": EMBEDDING_DIMENSIONS
        }
    });

    invoke_bedrock_model(bedrock_client, "amazon.titan-embed-image-v1", body).await
}

pub async fn extract_text_features(
    bedrock_client: &BedrockClient,
    text: String,
) -> Result<Vec<f32>, ApiError> {
    let body = json!({
        "inputText": text,
        "embeddingConfig": {
            "outputEmbeddingLength": EMBEDDING_DIMENSIONS
        }
    });

    invoke_bedrock_model(bedrock_client, "amazon.titan-embed-image-v1", body).await
}

pub async fn extract_text_features_multilingual(
    bedrock_client: &BedrockClient,
    text: &String,
) -> Result<Vec<f32>, ApiError> {
    let body = json!({
        "inputText": text,
        "dimensions": EMBEDDING_DIMENSIONS
    });

    invoke_bedrock_model(bedrock_client, "amazon.titan-embed-text-v2:0", body).await
}
