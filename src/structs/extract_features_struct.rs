use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct ExtractFeaturesPayload {
    pub image: String,
}

#[derive(Serialize)]
pub struct ExtractFeaturesBody {
    pub time: u64,
}

impl ExtractFeaturesBody {
    pub fn new(time: u64) -> Self {
        Self { time }
    }
}

pub enum ExtractFeaturesError {
    Unforseen,
}

impl IntoResponse for ExtractFeaturesError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ExtractFeaturesError::Unforseen => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };
        let body = Json(json!({
            "message": error_message,
        }));
        (status, body).into_response()
    }
}
