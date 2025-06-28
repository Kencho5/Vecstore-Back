use crate::prelude::*;

pub async fn nsfw_detector_handler(
    Extension(api_key): Extension<String>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<NsfwBody>, ApiError> {
    let total_start = Instant::now();

    let mut image_data: Option<Vec<u8>> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| ApiError::MissingData)?
    {
        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "image" | "file" => {
                image_data = Some(
                    field
                        .bytes()
                        .await
                        .map_err(|_| ApiError::MissingData)?
                        .to_vec(),
                );
            }
            _ => {} // Ignore unknown fields
        }
    }

    let image_data = image_data.ok_or(ApiError::MissingData)?;

    let image = load_image::load_image(image_data, 224);
    let nsfw =
        predict(&*state.nsfw_model, image.unwrap()).map_err(|_| ApiError::ImageProcessing)?;

    let total_time_ms = total_start.elapsed().as_millis() as u64;

    let validation_result = validate_nsfw_request(&state.pool, api_key)
        .await
        .map_err(|_| ApiError::Unforseen)?;

    let logs_task = BackgroundTask::SaveUsageLogs {
        pool: state.pool,
        user_id: validation_result.user_id,
    };

    if state.task_queue.send(logs_task).is_err() {
        eprintln!("Failed to send logs_task");
    }

    Ok(Json(NsfwBody {
        nsfw: nsfw == 1,
        time: total_time_ms,
        credits_left: validation_result.credits_left,
    }))
}

fn predict(model: &Model, input: Tensor) -> Result<i8, Box<dyn std::error::Error>> {
    let logits = model.forward(&input)?.squeeze(0)?;
    let scores = logits.to_vec1::<f32>()?;

    let pred = scores
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(idx, _)| idx)
        .unwrap_or(0);

    Ok(pred as i8)
}
