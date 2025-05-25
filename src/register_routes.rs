use crate::prelude::*;
use crate::routes::*;

pub fn create_router() -> Router<AppState> {
    Router::new().merge(pages())
}

fn pages() -> Router<AppState> {
    Router::new().route(
        "/extract-features",
        post(extract_features::extract_features_handler),
    )
}
