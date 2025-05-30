use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub email: String,
    pub name: String,
    pub password: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterPayload {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}

impl AuthResponse {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

pub enum AuthError {
    UserExists,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::UserExists => (StatusCode::BAD_REQUEST, "Email already exists"),
        };

        let body = Json(json!({
            "message": error_message,
        }));

        (status, body).into_response()
    }
}
