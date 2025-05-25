use crate::prelude::*;
use crate::routes::*;

pub fn create_router() -> Router<AppState> {
    Router::new().merge(pages()).merge(assets())
}

fn pages() -> Router<AppState> {
    Router::new().route("/", get(home::home_handler))
}
