use crate::prelude::*;
use crate::structs::google_struct::*;
use google_oauth::AsyncClient;

pub async fn verify_google_handler(
    State(state): State<AppState>,
    Json(payload): Json<VerifyGooglePayload>,
) -> Result<StatusCode, VerifyGoogleError> {
    let client = AsyncClient::new(
        "180388379586-s4el6paun1djkmlccn36lij94chnel0n.apps.googleusercontent.com",
    );

    let payload = client.validate_access_token(payload.token).await.unwrap(); // In production, remember to handle this error.

    // When we get the payload, that mean the id_token is valid.
    // Usually we use `sub` as the identifier for our user...
    println!("{:?}", &payload);
    Ok(StatusCode::OK)
}
