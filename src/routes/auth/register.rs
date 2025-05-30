use crate::{auth::insert_user::*, prelude::*};

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<AuthResponse>, AuthError> {
    payload.validate()?;

    let pwh = bcrypt::hash(&payload.password).unwrap();

    let user = User {
        email: payload.email,
        name: payload.name,
        password: Some(pwh),
    };

    insert_user(state.pool, user.clone())
        .await
        .map_err(|_| AuthError::UserExists)?;

    let token = create_token(user.email)
        .await
        .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(AuthResponse::new(token)))
}
