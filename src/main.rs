mod prelude;
mod register_routes;
mod routes;
mod structs;
mod utils;

use crate::prelude::*;
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let (model, clip_config) = load_model().unwrap();
    let qdrant_client = Arc::new(Qdrant::from_url("db.vecstore.app").build().unwrap());

    let state = AppState {
        model,
        clip_config,
        qdrant_client,
    };
    let app = register_routes::create_router()
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION]),
        )
        .with_state(state);

    println!("Server starting on 0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
