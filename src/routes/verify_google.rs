use crate::prelude::*;
use crate::structs::google_struct::*;

pub async fn verify_google_handler(
    State(state): State<AppState>,
    Json(payload): Json<VerifyGooglePayload>,
) -> Result<StatusCode, VerifyGoogleError> {
    let payload = state
        .google_client
        .validate_access_token(payload.token)
        .await
        .map_err(|_| VerifyGoogleError::InvalidToken)?;

    Ok(StatusCode::OK)
}
