use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct NsfwPayload {
    pub image: String,
}

#[derive(Serialize)]
pub struct NsfwBody {
    pub nsfw: bool,
    pub time: u64,
    pub credits_left: i32,
}

impl NsfwBody {
    pub fn new(nsfw: bool, time: u64, credits_left: i32) -> Self {
        Self {
            nsfw,
            credits_left,
            time,
        }
    }
}

pub enum NsfwError {
    Unforseen,
    ImageProcessing,
    MissingData,
}

impl IntoResponse for NsfwError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            NsfwError::Unforseen => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            NsfwError::ImageProcessing => (StatusCode::BAD_REQUEST, "Failed to process image"),
            NsfwError::MissingData => (StatusCode::BAD_REQUEST, "Missing payload data"),
        };

        let body = Json(json!({
            "message": error_message,
        }));

        (status, body).into_response()
    }
}
