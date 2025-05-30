use crate::prelude::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct VerifyGooglePayload {
    pub token: String,
}

pub enum VerifyGoogleError {
    InvalidToken,
    UserExists,
}

impl IntoResponse for VerifyGoogleError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            VerifyGoogleError::InvalidToken => {
                (StatusCode::BAD_REQUEST, "Failed to verify google token")
            }
            VerifyGoogleError::UserExists => (StatusCode::BAD_REQUEST, "Email already exists"),
        };

        let body = Json(json!({
            "message": error_message,
        }));

        (status, body).into_response()
    }
}
