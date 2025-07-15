use crate::auth::insert_user::insert_user;
use crate::prelude::*;
use crate::structs::google_struct::*;

pub async fn register_google_handler(
    State(state): State<AppState>,
    Json(payload): Json<GoogleAuthPayload>,
) -> Result<Json<AuthResponse>, AuthError> {
    let claims = state
        .google_client
        .validate_access_token(&payload.token)
        .await
        .map_err(|_| AuthError::InvalidToken)?;

    let user = User {
        email: claims.email.expect("missing email from Google payload"),
        name: claims.name.expect("missing name from Google payload"),
        company: None,
        password: None,
    };

    let user_id = insert_user(state.pool, user.clone())
        .await
        .map_err(|_| AuthError::UserExists)?;

    let token = create_token(user_id, &user.email, user.name)
        .await
        .map_err(|_| AuthError::InvalidToken)?;

    let send_feedback_email = BackgroundTask::SendFeedbackEmail {
        client: state.ses_client,
        recipient: user.email,
    };

    if state.task_queue.send(send_feedback_email).is_err() {
        eprintln!("Failed to send user action task");
    }

    Ok(Json(AuthResponse { token }))
}
