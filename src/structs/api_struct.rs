use crate::prelude::*;

//IMAGE
#[derive(Deserialize, Serialize)]
pub struct InsertImagePayload {
    pub image: String,
    pub database: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct InsertImageBody {
    pub time: String,
}

//TEXT
#[derive(Deserialize, Serialize)]
pub struct InsertTextPayload {
    pub text: String,
    pub database: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct InsertTextResponse {
    pub time: String,
}

//SEARCH
#[derive(Deserialize, Serialize)]
pub struct SearchPayload {
    pub text: Option<String>,
    pub image: Option<String>,
    pub database: String,
    pub metadata: Option<serde_json::Value>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
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
    pub vector_id: String,
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
    pub labels: Vec<ModerationLabel>,
}

#[derive(Serialize)]
pub struct ModerationLabel {
    pub label: String,
    pub confidence: String,
}

//USER VALIDATION
#[derive(Deserialize, Serialize, Clone)]
pub struct UserValidationResult {
    pub user_id: i32,
    pub region: String,
    pub credits_left: i32,
    pub db_type: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UserCacheResult {
    pub user_id: i32,
    pub region: String,
    pub db_type: String,
}

pub struct UserRegionResult {
    pub user_id: i32,
    pub region: String,
}

pub struct UserNsfwValidationResult {
    pub user_id: i32,
    pub credits_left: i32,
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
        };

        let body = Json(json!({
            "message": error_message,
        }));

        (status, body).into_response()
    }
}
