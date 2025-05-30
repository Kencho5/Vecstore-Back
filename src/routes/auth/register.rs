use crate::prelude::*;

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> Result<AuthResponse, AuthError> {
    Ok(AuthResponse::new("".to_string()))
}
