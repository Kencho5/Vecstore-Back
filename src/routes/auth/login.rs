use crate::{auth::check_user::check_user, prelude::*, structs::google_struct::*};

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<GoogleAuthPayload>,
) -> Result<Json<AuthResponse>, AuthError> {
    let claims = state
        .google_client
        .validate_access_token(&payload.token)
        .await
        .map_err(|_| AuthError::InvalidToken)?;

    let user = User {
        name: claims.name.expect("missing name from Google payload"),
        email: claims.email.expect("missing email from Google payload"),
        password: None,
    };

    check_user(state.pool, user.clone())
        .await
        .map_err(|_| AuthError::UserNotFound)?;

    let token = create_token(user.email, user.name)
        .await
        .map_err(|_| AuthError::InvalidToken)?;

    Ok(Json(AuthResponse::new(token)))
}
