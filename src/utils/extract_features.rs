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
    image_bytes: Vec<u8>,
) -> Result<Vec<f32>, ApiError> {
    let resized_image_data = resize_image(image_bytes)?;
    let final_base64 = base64::engine::general_purpose::STANDARD.encode(&resized_image_data);

    let body = json!({
        "inputImage": final_base64,
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
