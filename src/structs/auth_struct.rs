use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone)]
pub struct User {
    pub email: String,
    pub name: String,
    pub company: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct UserResponse {
    pub id: i32,
    pub email: String,
    pub name: String,
    #[serde(skip_serializing)]
    pub password: Option<String>,
}

//REGISTER
#[derive(Deserialize, Serialize)]
pub struct RegisterPayload {
    pub email: String,
    pub name: String,
    pub company: Option<String>,
    pub password: String,
}

//LOGIN
#[derive(Deserialize, Serialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: i32,
    pub email: String,
    pub name: String,
    pub company: String,
    pub exp: i64,
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
    UserNotFound,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::UserExists => (StatusCode::CONFLICT, "Email already exists"),
            AuthError::UserNotFound => (StatusCode::UNAUTHORIZED, "Invalid email or password"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create jwt token",
            ),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Unauthorized"),
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
                if value.is_null() {
                    continue;
                }
                if value.as_str().map_or(false, |s| s.is_empty()) {
                    return Err(AuthError::MissingCredentials);
                }
            }
        }

        Ok(())
    }
}
