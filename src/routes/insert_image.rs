use crate::prelude::*;

pub async fn insert_image_handler(
    Extension(api_key): Extension<String>,
    State(state): State<AppState>,
    Json(payload): Json<InsertImagePayload>,
) -> Result<Json<InsertImageBody>, ApiError> {
    let total_start = Instant::now();

    let validation_result =
        validate_user_and_increment(&state.pool, api_key, &payload.database).await?;

    let insert_task = BackgroundTask::InsertImageVectors {
        user_id: validation_result.user_id,
        base64_image: payload.image,
        metadata: payload.metadata,
        database: payload.database,
        region: validation_result.region,
    };

    let logs_task = BackgroundTask::SaveUsageLogs {
        user_id: validation_result.user_id,
    };

    if state.task_queue.send(insert_task).is_err() {
        eprintln!("Failed to send insert_task");
    }

    if state.task_queue.send(logs_task).is_err() {
        eprintln!("Failed to send logs_task");
    }

    let total_time_ms = total_start.elapsed().as_millis() as u64;

    Ok(Json(InsertImageBody {
        time: format!("{}ms", total_time_ms),
        credits_left: validation_result.credits_left,
    }))
}
