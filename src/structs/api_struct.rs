use crate::prelude::*;

//IMAGE
#[derive(Serialize)]
pub struct InsertImageBody {
    pub time: String,
    pub credits_left: i32,
}

//TEXT
#[derive(Deserialize, Serialize)]
pub struct InsertTextPayload {
    pub text: String,
    pub database: String,
    pub metadata: Option<String>,
}

#[derive(Serialize)]
pub struct InsertTextResponse {
    pub time: String,
    pub credits_left: i32,
}

//SEARCH
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
    pub credits_left: i32,
}

#[derive(Serialize)]
pub struct SearchResults {
    pub matches: Vec<SearchMatch>,
}

#[derive(Serialize)]
pub struct SearchMatch {
    pub id: String,
    pub score: String,
    pub metadata: Option<HashMap<String, String>>,
}

//NSFW
#[derive(Deserialize, Serialize)]
pub struct NsfwPayload {
    pub image: String,
}

#[derive(Deserialize, Serialize)]
pub struct NsfwFile {
    pub nsfw: Vec<f32>,
    pub not_nsfw: Vec<f32>,
}

#[derive(Serialize)]
pub struct NsfwBody {
    pub nsfw: bool,
    pub time: u64,
    pub credits_left: i32,
    pub labels: Vec<ModerationLabel>,
}

#[derive(Serialize)]
pub struct ModerationLabel {
    pub label: String,
    pub confidence: String,
}

#[derive(Debug)]
pub enum ApiError {
    Unforseen,
    ImageProcessing,
    ModelInference,
    DatabaseError,
    DatabaseNotFound,
    DatabaseInsert,
    MissingData,
    InvalidApiKey,
    RequestLimitExceeded,
    InvalidMetadata,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::Unforseen => {
                (StatusCode::BAD_REQUEST, "Unforseen error. Contact support")
            }
            ApiError::ImageProcessing => {
                (StatusCode::BAD_REQUEST, "Failed to process image")
            }
            ApiError::ModelInference => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Model inference failed")
            }
            ApiError::DatabaseError=> (
                StatusCode::BAD_REQUEST,
                "Database error",
            ),
            ApiError::DatabaseNotFound => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Database not found",
            ),
            ApiError::DatabaseInsert => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to insert into database",
            ),
            ApiError::MissingData => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Missing api data")
            }
            ApiError::InvalidApiKey => (StatusCode::UNAUTHORIZED, "Invalid API key"),
            ApiError::RequestLimitExceeded => (
                StatusCode::UNAUTHORIZED,
                "Monthly API request limit exceeded. Upgrade your plan or contact sales to increase your limit.",
            ),
            ApiError::InvalidMetadata => (
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
