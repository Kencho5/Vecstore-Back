use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct AddDbPayload {
    pub db_type: String,
    pub name: String,
    pub region: String,
}

pub enum AddDbError {
    Unforseen,
    MissingDbData,
    Unauthorized,
}

impl IntoResponse for AddDbError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AddDbError::MissingDbData => {
                (StatusCode::BAD_REQUEST, "Missing database creation data")
            }
            AddDbError::Unforseen => (StatusCode::BAD_REQUEST, "Failed to create database"),
            AddDbError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
        };

        let body = Json(json!({
            "message": error_message,
        }));

        (status, body).into_response()
    }
}
