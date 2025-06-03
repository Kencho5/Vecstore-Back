use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct AddDbPayload {
    pub db_type: String,
    pub name: String,
    pub region: String,
}

pub enum AddDbError {
    Unforseen,
}

impl IntoResponse for AddDbError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AddDbError::Unforseen => (StatusCode::BAD_REQUEST, "Failed to create database"),
        };

        let body = Json(json!({
            "message": error_message,
        }));

        (status, body).into_response()
    }
}
