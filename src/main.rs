mod prelude;
mod register_routes;
mod routes;
mod structs;
mod utils;

use crate::prelude::*;
use tracing::Level;

#[tokio::main]
async fn main() {
    let app_start = Instant::now();

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    println!("Loading model...");
    let model_load_start = Instant::now();

    let device = Device::Cpu;
    let clip_config = clip::ClipConfig::vit_base_patch32();
    let model_path = load_model().expect("Failed to get model");
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[&model_path], DType::F32, &device) };
    let model = clip::ClipModel::new(vb.unwrap(), &clip_config).expect("unable to load model");

    let model_load_time = model_load_start.elapsed().as_millis();
    println!("Model loaded in: {}ms", model_load_time);

    let state = AppState { model, clip_config };
    let app = register_routes::create_router()
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION]),
        )
        .with_state(state);

    let total_startup_time = app_start.elapsed().as_millis();
    println!("Total startup time: {}ms", total_startup_time);
    println!("Server starting on 0.0.0.0:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
