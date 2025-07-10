use crate::prelude::*;

pub async fn insert_text_handler(
    Extension(api_key): Extension<String>,
    State(state): State<AppState>,
    Json(payload): Json<InsertTextPayload>,
) -> Result<Json<InsertTextResponse>, ApiError> {
    let cached_user = get_cached_user(&state, api_key, &payload.database).await?;

    let total_start = Instant::now();

    let insert_task = BackgroundTask::InsertTextVectors {
        user_id: cached_user.user_id,
        text: payload.text,
        database: payload.database.clone(),
        region: cached_user.region,
        metadata: payload.metadata,
    };

    let user_action_task = BackgroundTask::ProcessUserAction {
        user_id: cached_user.user_id,
        database: payload.database.clone(),
    };

    if state.task_queue.send(insert_task).is_err() {
        eprintln!("Failed to send insert_task");
    }

    if state.task_queue.send(user_action_task).is_err() {
        eprintln!("Failed to send user action task");
    }

    let total_time_ms = total_start.elapsed().as_millis() as u64;

    Ok(Json(InsertTextResponse {
        time: format!("{}ms", total_time_ms),
    }))
}
