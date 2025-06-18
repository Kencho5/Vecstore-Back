use crate::{auth::check_user::check_user, prelude::*, structs::google_struct::*};

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<AuthResponse>, AuthError> {
    payload.validate()?;

    let db_user = check_user(&state.pool, payload.email, Some(payload.password))
        .await
        .map_err(|_| AuthError::UserNotFound)?;

    let token = create_token(db_user.id, db_user.email, db_user.name, db_user.plan_names)
        .await
        .map_err(|_| AuthError::InvalidToken)?;

    Ok(Json(AuthResponse::new(token)))
}

pub async fn login_google_handler(
    State(state): State<AppState>,
    Json(payload): Json<GoogleAuthPayload>,
) -> Result<Json<AuthResponse>, AuthError> {
    let claims = state
        .google_client
        .validate_access_token(&payload.token)
        .await
        .map_err(|_| AuthError::InvalidToken)?;

    let db_user = check_user(&state.pool, claims.email.unwrap(), None)
        .await
        .map_err(|_| AuthError::UserNotFound)?;

    let token = create_token(db_user.id, db_user.email, db_user.name, db_user.plan_names)
        .await
        .map_err(|_| AuthError::InvalidToken)?;

    Ok(Json(AuthResponse::new(token)))
}

pub async fn refresh_token(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> Result<Json<AuthResponse>, AuthError> {
    let db_user = get_user_details(&state.pool, claims.email)
        .await
        .map_err(|e| {
            dbg!(e);
            AuthError::UserNotFound
        })?;

    let token = create_token(db_user.id, db_user.email, db_user.name, db_user.plan_names)
        .await
        .map_err(|_| AuthError::InvalidToken)?;

    Ok(Json(AuthResponse::new(token)))
}
