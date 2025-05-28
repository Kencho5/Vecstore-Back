use crate::prelude::*;
use crate::routes::*;

pub fn create_router() -> Router<AppState> {
    Router::new().merge(pages())
}

fn pages() -> Router<AppState> {
    Router::new()
        .route("/insert", post(insert_image::insert_image_handler))
        .route("/search", post(search_image::search_image_handler))
}
