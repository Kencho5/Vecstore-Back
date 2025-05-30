use crate::prelude::*;

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<AuthResponse>, AuthError> {
    payload.validate()?;

    Ok(Json(AuthResponse::new("".to_string())))
}
