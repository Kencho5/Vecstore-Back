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
        company: payload.company,
        password: Some(pwh),
    };

    let user_id = insert_user(state.pool, user.clone())
        .await
        .map_err(|_| AuthError::UserExists)?;

    let token = create_token(user_id, user.email, user.name)
        .await
        .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(AuthResponse { token }))
}
