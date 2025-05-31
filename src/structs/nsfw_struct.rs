use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct NsfwPayload {
    pub image: String,
}

#[derive(Serialize)]
pub struct NsfwBody {
    pub nsfw: bool,
    pub time: u64,
}

impl NsfwBody {
    pub fn new(nsfw: bool, time: u64) -> Self {
        Self { nsfw, time }
    }
}

pub enum NsfwError {
    ImageProcessing,
}

impl IntoResponse for NsfwError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            NsfwError::ImageProcessing => (StatusCode::BAD_REQUEST, "Failed to process image"),
        };

        let body = Json(json!({
            "message": error_message,
        }));

        (status, body).into_response()
    }
}
