use crate::prelude::*;
use crate::routes::*;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .merge(api_routes())
        .merge(dashboard_routes())
        .merge(auth())
        .route_layer(middleware::from_fn(validate_headers))
        .merge(health())
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/insert", post(insert_image::insert_image_handler))
        .route("/search", post(search_image::search_image_handler))
        .route("/nsfw", post(nsfw_detector::nsfw_detector_handler))
}

fn dashboard_routes() -> Router<AppState> {
    Router::new()
        .route("/add-db", post(add_db::add_db_handler))
        .route("/get-dbs", get(get_dbs::get_dbs_handler))
        .route("/index-data", get(get_index_data::index_data_handler))
}

fn auth() -> Router<AppState> {
    Router::new()
        .route(
            "/register-google",
            post(register_google::register_google_handler),
        )
        .route("/register", post(register::register_handler))
        .route("/login", post(login::login_handler))
}

fn health() -> Router<AppState> {
    Router::new().route("/health", get(health::health_handler))
}
