use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct ExtractFeaturesPayload {
    pub image: String,
}

#[derive(Serialize)]
pub struct ExtractFeaturesBody {
    pub vector: String,
}

impl ExtractFeaturesBody {
    pub fn new(vector: String) -> Self {
        Self { vector }
    }
}

#[derive(Debug)]
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
