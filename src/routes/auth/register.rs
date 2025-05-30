use crate::prelude::*;

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> Result<StatusCode, AuthError> {
    Ok(StatusCode::OK)
}
