use crate::prelude::*;
use crate::routes::*;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .merge(dashboard_routes())
        .merge(api_routes())
        .merge(auth_routes())
        .merge(payment_routes())
        .merge(health())
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/insert-image", post(insert_image::insert_image_handler))
        .route("/insert-text", post(insert_text::insert_text_handler))
        .route("/search", post(search::search_handler))
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
        .route(
            "/delete-api-key",
            delete(delete_api_key::delete_api_key_handler),
        )
        .route(
            "/payment-methods",
            get(payment_methods::payment_methods_handler),
        )
        .route("/portal-url", get(portal_url::portal_url_handler))
        .route("/usage", get(usage::usage_handler))
        .route_layer(middleware::from_fn(validate_headers))
}

fn auth_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/register-google",
            post(register_google::register_google_handler),
        )
        .route("/register", post(register::register_handler))
        .route("/login-google", post(login::login_google_handler))
        .route("/login", post(login::login_handler))
}

fn payment_routes() -> Router<AppState> {
    Router::new().route(
        "/payments-webhook",
        post(payments_webhook::payments_webhook_handler),
    )
}

fn health() -> Router<AppState> {
    Router::new().route("/health", get(health::health_handler))
}
