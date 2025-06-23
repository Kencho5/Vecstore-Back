use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct SearchImagePayload {
    pub text: Option<String>,
    pub image: Option<String>,
    pub database: String,
    pub metadata: Option<String>,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub results: Vec<SearchMatch>,
    pub time: String,
}

#[derive(Serialize)]
pub struct SearchResults {
    pub matches: Vec<SearchMatch>,
}

#[derive(Serialize)]
pub struct SearchMatch {
    pub score: String,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug)]
pub enum SearchImageError {
    Unforseen,
    ModelInference,
    MissingData,
    InvalidApiKey,
    InvalidMetadata,
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
            SearchImageError::MissingData => (StatusCode::BAD_REQUEST, "Missing search data"),
            SearchImageError::InvalidApiKey => (StatusCode::BAD_REQUEST, "Invalid api key"),
            SearchImageError::InvalidMetadata => (
                StatusCode::BAD_REQUEST,
                "Invalid metadata format. Must be valid JSON.",
            ),
        };

        let body = Json(json!({
            "message": error_message,
        }));

        (status, body).into_response()
    }
}
