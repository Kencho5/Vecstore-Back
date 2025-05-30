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
    MissingCredentials,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::UserExists => (StatusCode::BAD_REQUEST, "Email already exists"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
        };

        let body = Json(json!({
            "message": error_message,
        }));

        (status, body).into_response()
    }
}

pub trait ValidatePayload {
    fn validate(&self) -> Result<(), AuthError>;
}

impl<T> ValidatePayload for T
where
    T: Serialize,
{
    fn validate(&self) -> Result<(), AuthError> {
        let json_value = serde_json::to_value(self).unwrap();

        if let serde_json::Value::Object(map) = json_value {
            for (_, value) in map.iter() {
                if value.as_str().map_or(true, |s| s.is_empty()) {
                    return Err(AuthError::MissingCredentials);
                }
            }
        }

        Ok(())
    }
}
