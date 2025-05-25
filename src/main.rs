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

    println!("Loading model...");
    let device = Device::Cpu;
    let clip_config = clip::ClipConfig::vit_base_patch32();

    // Load model from local disk
    let model_path = load_model().expect("Failed to get model");

    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[&model_path], DType::F32, &device) };
    let model = clip::ClipModel::new(vb.unwrap(), &clip_config).expect("unable to load model");

    println!("Model Loaded");

    let state = AppState { model, clip_config };

    let app = register_routes::create_router()
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION]),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
