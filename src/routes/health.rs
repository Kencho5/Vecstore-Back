use crate::prelude::*;

pub async fn health_handler() -> impl IntoResponse {
    StatusCode::OK
}
