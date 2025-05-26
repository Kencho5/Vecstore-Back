use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct InsertImagePayload {
    pub image: String,
}

#[derive(Serialize)]
pub struct InsertImageBody {
    pub time: u64,
}

impl InsertImageBody {
    pub fn new(time: u64) -> Self {
        Self { time }
    }
}

pub enum InsertImageError {
    Unforseen,
}

impl IntoResponse for InsertImageError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            InsertImageError::Unforseen => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };
        let body = Json(json!({
            "message": error_message,
        }));
        (status, body).into_response()
    }
}
