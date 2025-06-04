mod auth;
mod loaders;
mod middleware;
mod prelude;
mod register_routes;
mod routes;
mod structs;
mod utils;

use crate::prelude::*;
use tracing::Level;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let (clip_model, clip_config) = load_model::load_clip_model().unwrap();
    let nsfw_model = load_model::load_nsfw_model().unwrap();
    let pinecone = init_pinecone::init_pinecone().await;
    let tokenizer = get_tokenizer(None).expect("Failed to get tokenizer");
    let pool = init_db::init_db().await;
    let google_client =
        AsyncClient::new(env::var("GOOGLE_CLIENT_ID").expect("Google client id not found"));
    let (tx, rx) = mpsc::unbounded_channel::<BackgroundTask>();

    let state = AppState {
        clip_model,
        clip_config,
        pinecone,
        tokenizer,
        pool,
        google_client,
        nsfw_model,
        task_queue: tx,
    };

    let worker_state = WorkerState {
        pinecone: state.pinecone.clone(),
    };

    tokio::spawn(async move {
        process_task_queue(rx, worker_state).await;
    });

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
