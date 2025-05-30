use crate::auth::insert_user::insert_user;
use crate::prelude::*;
use crate::structs::google_struct::*;

pub async fn register_google_handler(
    State(state): State<AppState>,
    Json(payload): Json<VerifyGooglePayload>,
) -> Result<StatusCode, VerifyGoogleError> {
    let payload = state
        .google_client
        .validate_access_token(payload.token)
        .await
        .map_err(|_| VerifyGoogleError::InvalidToken)?;

    let user = User {
        name: payload.name.expect("missing name from Google payload"),
        email: payload.email.expect("missing email from Google payload"),
        password: None,
    };

    insert_user(state.pool, user)
        .await
        .map_err(|_| VerifyGoogleError::UserExists)?;

    Ok(StatusCode::OK)
}
