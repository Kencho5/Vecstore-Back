use crate::{
    prelude::*,
    structs::nsfw_struct::{NsfwBody, NsfwError, NsfwPayload},
};

pub async fn nsfw_detector_handler(
    State(state): State<AppState>,
    Json(payload): Json<NsfwPayload>,
) -> Result<Json<NsfwBody>, NsfwError> {
    let total_start = Instant::now();

    let image = load_image::load_image(payload.image, 224);
    let nsfw = predict(state.nsfw_model, image.unwrap()).map_err(|_| NsfwError::ImageProcessing)?;

    let total_time_ms = total_start.elapsed().as_millis() as u64;
    println!("Total NSFW handler time: {}ms", total_time_ms);

    Ok(Json(NsfwBody::new(nsfw == 1, total_time_ms)))
}

fn predict(model: Model, input: Tensor) -> Result<i8, Box<dyn std::error::Error>> {
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
