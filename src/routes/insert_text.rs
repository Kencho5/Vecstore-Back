use crate::{prelude::*, structs::insert_struct::*};

pub async fn insert_text_handler(
    Extension(api_key): Extension<String>,
    State(state): State<AppState>,
    Json(payload): Json<InsertTextPayload>,
) -> Result<(), InsertError> {
    let validation_result =
        validate_user_and_increment(&state.pool, api_key, payload.database.clone()).await?;

    let user_id = validation_result.user_id;

    let text_vectors = extract_text_features(&state, payload.text)
        .await
        .map_err(|_| InsertError::ModelInference)?;

    let insert_task = BackgroundTask::InsertVectors {
        user_id,
        vectors: text_vectors,
        filename: None,
        database: payload.database,
        region: validation_result.region,
        metadata: payload.metadata,
    };

    if state.task_queue.send(insert_task).is_err() {
        eprintln!("Failed to send insert_task");
    }

    Ok(())
}
