use crate::{prelude::*, structs::insert_struct::*};

pub async fn insert_text_handler(
    Extension(api_key): Extension<String>,
    State(state): State<AppState>,
    Json(payload): Json<InsertTextPayload>,
) -> Result<(), InsertError> {
    let user_id = get_user_key(&state.pool, api_key, "Text Search".to_string()).await?;

    let text_vectors = extract_text_features(&state, payload.text)
        .await
        .map_err(|_| InsertError::ModelInference)?;

    let insert_task = BackgroundTask::InsertVectors {
        user_id,
        vectors: text_vectors,
        filename: None,
        database: payload.database,
    };

    if state.task_queue.send(insert_task).is_err() {
        eprintln!("Failed to send insert_task");
    }

    Ok(())
}
