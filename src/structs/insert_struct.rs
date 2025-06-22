use crate::prelude::*;

//IMAGE
#[derive(Deserialize, Serialize)]
pub struct SearchImagePayload {
    pub text: String,
}

#[derive(Serialize)]
pub struct InsertImageBody {
    pub time: String,
}

impl InsertImageBody {
    pub fn new(time: String) -> Self {
        Self { time }
    }
}

//TEXT
#[derive(Deserialize, Serialize)]
pub struct InsertTextPayload {
    pub text: String,
    pub database: String,
}

#[derive(Debug)]
pub enum InsertError {
    ImageProcessing,
    ModelInference,
    DatabaseNotFound,
    DatabaseInsert,
    MissingData,
    InvalidApiKey,
    InvalidSubscription,
    RequestLimitExceeded,
}

impl IntoResponse for InsertError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            InsertError::ImageProcessing => {
                (StatusCode::BAD_REQUEST, "Failed to process image")
            }
            InsertError::ModelInference => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Model inference failed")
            }
            InsertError::DatabaseNotFound => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Database not found",
            ),
            InsertError::DatabaseInsert => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to insert into database",
            ),
            InsertError::MissingData => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Missing api data")
            }
            InsertError::InvalidApiKey => (StatusCode::UNAUTHORIZED, "Invalid API key"),
            InsertError::InvalidSubscription => (
                StatusCode::UNAUTHORIZED,
                "No active subscription found for this user",
            ),
            InsertError::RequestLimitExceeded => (
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
