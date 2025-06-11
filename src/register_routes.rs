use crate::prelude::*;
use crate::routes::*;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .merge(dashboard_routes())
        .merge(api_routes())
        .merge(auth())
        .merge(health())
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/insert", post(insert_image::insert_image_handler))
        .route("/search", post(search_image::search_image_handler))
        .route("/nsfw", post(nsfw_detector::nsfw_detector_handler))
        .route_layer(middleware::from_fn(api_middleware))
}

fn dashboard_routes() -> Router<AppState> {
    Router::new()
        .route("/add-db", post(add_db::add_db_handler))
        .route("/get-dbs", get(get_dbs::get_dbs_handler))
        .route("/get-db", post(get_dbs::get_db_handler))
        .route("/index-data", post(get_index_data::index_data_handler))
        .route(
            "/create-api-key",
            post(create_api_key::create_api_key_handler),
        )
        .route("/get-api-keys", get(get_api_keys::get_api_keys_handler))
        .route_layer(middleware::from_fn(validate_headers))
}

fn auth() -> Router<AppState> {
    Router::new()
        .route(
            "/register-google",
            post(register_google::register_google_handler),
        )
        .route("/register", post(register::register_handler))
        .route("/login-google", post(login::login_google_handler))
}

fn health() -> Router<AppState> {
    Router::new().route("/health", get(health::health_handler))
}
