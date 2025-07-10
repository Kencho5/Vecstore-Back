use crate::prelude::*;
use base64::Engine;

pub async fn nsfw_detector_handler(
    Extension(api_key): Extension<String>,
    State(state): State<AppState>,
    Json(payload): Json<NsfwPayload>,
) -> Result<Json<NsfwBody>, ApiError> {
    let total_start = Instant::now();

    // Decode base64 image
    let image_data = base64::engine::general_purpose::STANDARD
        .decode(&payload.image)
        .map_err(|_| ApiError::ImageProcessing)?;

    let image = aws_sdk_rekognition::types::Image::builder()
        .bytes(image_data.into())
        .build();

    let result = state
        .rekognition_client
        .detect_moderation_labels()
        .image(image)
        .send()
        .await
        .map_err(|_| ApiError::Unforseen)?;

    let moderation_labels = result.moderation_labels();

    let labels: Vec<ModerationLabel> = moderation_labels
        .iter()
        .map(|label| ModerationLabel {
            label: label.name().unwrap_or("Unknown").to_string(),
            confidence: format!("{:.1}%", label.confidence().unwrap_or(0.0)),
        })
        .collect();

    let nsfw = moderation_labels
        .iter()
        .any(|label| label.name().map_or(false, is_nsfw_label));

    let total_time_ms = total_start.elapsed().as_millis() as u64;
    let validation_result = validate_nsfw_request(&state.pool, api_key).await?;

    let logs_task = BackgroundTask::SaveUsageLogs {
        user_id: validation_result.user_id,
    };

    if state.task_queue.send(logs_task).is_err() {
        eprintln!("Failed to send logs_task");
    }

    Ok(Json(NsfwBody {
        nsfw,
        time: total_time_ms,
        labels,
    }))
}

fn is_nsfw_label(label_name: &str) -> bool {
    matches!(
        label_name,
        "Explicit Nudity" | "Nudity" | "Sexual Activity" | "Graphic Violence" | "Violence"
    )
}
