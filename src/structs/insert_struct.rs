use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct InsertImagePayload {
    pub image: String,
    pub filename: String,
    pub database: String,
}

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

#[derive(Debug)]
pub enum InsertImageError {
    ImageProcessing,
    ModelInference,
    DatabaseConnection,
    DatabaseInsert,
    MissingData,
    InvalidApiKey,
}

impl IntoResponse for InsertImageError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            InsertImageError::ImageProcessing => {
                (StatusCode::BAD_REQUEST, "Failed to process image")
            }
            InsertImageError::ModelInference => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Model inference failed")
            }
            InsertImageError::DatabaseConnection => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Database connection failed",
            ),
            InsertImageError::DatabaseInsert => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to insert into database",
            ),
            InsertImageError::MissingData => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Missing api data")
            }
            InsertImageError::InvalidApiKey => (StatusCode::UNAUTHORIZED, "Api key invalid"),
        };

        let body = Json(json!({
            "message": error_message,
        }));

        (status, body).into_response()
    }
}
