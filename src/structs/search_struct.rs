use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct SearchImagePayload {
    pub text: String,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub matches: Vec<SearchMatch>,
}

#[derive(Serialize)]
pub struct SearchMatch {
    pub id: String,
    pub score: f32,
}

#[derive(Debug)]
pub enum SearchImageError {
    Unforseen,
    ModelInference,
}

impl IntoResponse for SearchImageError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            SearchImageError::ModelInference => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Model inference failed")
            }
            SearchImageError::Unforseen => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        let body = Json(json!({
            "message": error_message,
        }));

        (status, body).into_response()
    }
}
