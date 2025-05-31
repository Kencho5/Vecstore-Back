use crate::auth::insert_user::insert_user;
use crate::prelude::*;
use crate::structs::google_struct::*;

pub async fn register_google_handler(
    State(state): State<AppState>,
    Json(payload): Json<VerifyGooglePayload>,
) -> Result<Json<AuthResponse>, VerifyGoogleError> {
    let claims = state
        .google_client
        .validate_access_token(&payload.token)
        .await
        .map_err(|_| VerifyGoogleError::InvalidToken)?;

    let user = User {
        name: claims.name.expect("missing name from Google payload"),
        email: claims.email.expect("missing email from Google payload"),
        password: None,
    };

    insert_user(state.pool, user.clone())
        .await
        .map_err(|_| VerifyGoogleError::UserExists)?;

    let token = create_token(user.email, user.name)
        .await
        .map_err(|_| VerifyGoogleError::InvalidToken)?;

    Ok(Json(AuthResponse::new(token)))
}
