use crate::prelude::*;
use crate::routes::*;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .merge(api_routes())
        .merge(auth())
        .merge(health())
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/insert", post(insert_image::insert_image_handler))
        .route("/search", post(search_image::search_image_handler))
}

fn auth() -> Router<AppState> {
    Router::new().route("/verify-google", post(verify_google::verify_google_handler))
}

fn health() -> Router<AppState> {
    Router::new().route("/health", get(health::health_handler))
}
