use crate::{prelude::*, structs::dashboard_struct::*};
use pdf_extract::extract_text_from_mem;

pub async fn upload_files_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Json(payload): Json<UploadFilesPayload>,
) -> Result<StatusCode, DashboardError> {
    let file_count = payload.files.len();

    deduct_credits(&state.pool, claims.user_id, file_count).await?;

    let pool = state
        .neon_pools
        .get_pool_by_region(&payload.region)
        .unwrap();

    match payload.files_type.as_str() {
        "image" => {
            upload_image(
                payload.files,
                pool,
                claims.user_id,
                state.bedrock_client,
                payload.name,
            )
            .await?
        }
        "pdf" => {
            upload_pdfs(
                payload.files,
                pool,
                claims.user_id,
                state.bedrock_client,
                payload.name,
            )
            .await?
        }
        "text" => {
            upload_text(
                payload.files,
                pool,
                claims.user_id,
                state.bedrock_client,
                payload.name,
            )
            .await?
        }
        _ => {}
    }

    Ok(StatusCode::OK)
}

async fn upload_image(
    files: Vec<File>,
    pool: &PgPool,
    user_id: i32,
    bedrock_client: BedrockClient,
    database: String,
) -> Result<(), DashboardError> {
    for file in files {
        let image_bytes = base64::engine::general_purpose::STANDARD
            .decode(&file.data)
            .map_err(|_| DashboardError::Unforseen)?;
        let vectors = extract_image_features(&bedrock_client, image_bytes)
            .await
            .unwrap();

        let metadata = Some(serde_json::json!({
            "filename": file.name
        }));

        insert_vectors(pool, user_id, vectors, metadata, database.clone())
            .await
            .map_err(|_| DashboardError::Unforseen)?;
    }
    Ok(())
}

async fn upload_pdfs(
    files: Vec<File>,
    pool: &PgPool,
    user_id: i32,
    bedrock_client: BedrockClient,
    database: String,
) -> Result<(), DashboardError> {
    for file in files {
        let pdf_bytes = base64::engine::general_purpose::STANDARD
            .decode(&file.data)
            .map_err(|_| DashboardError::Unforseen)?;

        let text = extract_text_from_mem(&pdf_bytes).map_err(|_| DashboardError::Unforseen)?;
        let clean_text = text.replace('\n', " ");

        let vectors = extract_text_features_multilingual(&bedrock_client, clean_text)
            .await
            .unwrap();

        let metadata = Some(serde_json::json!({
            "filename": file.name
        }));

        insert_vectors(pool, user_id, vectors, metadata, database.clone())
            .await
            .map_err(|_| DashboardError::Unforseen)?;
    }
    Ok(())
}

async fn upload_text(
    files: Vec<File>,
    pool: &PgPool,
    user_id: i32,
    bedrock_client: BedrockClient,
    database: String,
) -> Result<(), DashboardError> {
    for file in files {
        let vectors = extract_text_features_multilingual(&bedrock_client, file.data)
            .await
            .unwrap();

        let metadata = Some(serde_json::json!({
            "filename": file.name
        }));

        insert_vectors(pool, user_id, vectors, metadata, database.clone())
            .await
            .map_err(|_| DashboardError::Unforseen)?;
    }
    Ok(())
}
