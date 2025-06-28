use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct VerifyEmailPayload {
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct VerifyCodePayload {
    pub email: String,
    pub code: i16,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct EmailRecord {
    pub code: i16,
    pub expiry: NaiveDateTime,
}

pub enum VerifyEmailError {
    InvalidEmail,
    InsertFailed,
    InvalidCode,
    CodeExpired,
}

impl IntoResponse for VerifyEmailError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            VerifyEmailError::InvalidEmail => (StatusCode::BAD_REQUEST, "Invalid email address"),
            VerifyEmailError::InsertFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unforseen error. contact support",
            ),
            VerifyEmailError::InvalidCode => (StatusCode::BAD_REQUEST, "Invalid code"),
            VerifyEmailError::CodeExpired => (StatusCode::BAD_REQUEST, "Code Expired"),
        };
        let body = Json(json!({
            "message": error_message,
        }));
        (status, body).into_response()
    }
}
