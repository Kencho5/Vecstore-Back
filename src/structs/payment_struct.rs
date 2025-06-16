use crate::prelude::*;

// PAYMENT CREATED
#[derive(Deserialize, Serialize)]
pub struct PaymentCreatedPayload {
    pub event_id: String,
}

pub enum PaymentError {
    Unforseen,
}

impl IntoResponse for PaymentError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            PaymentError::Unforseen => {
                (StatusCode::BAD_REQUEST, "Unforseen error. Contact support")
            }
        };

        let body = Json(json!({
            "message": error_message,
        }));

        (status, body).into_response()
    }
}
