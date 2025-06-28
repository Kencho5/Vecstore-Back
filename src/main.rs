#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

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

    let pinecone_indexes = init_pinecone::init_pinecone().await;
    let pinecone_indexes = Arc::new(Mutex::new(pinecone_indexes));
    let pool = init_db::init_db().await;
    let google_client =
        AsyncClient::new(env::var("GOOGLE_CLIENT_ID").expect("Google client id not found"));
    let (tx, rx) = mpsc::unbounded_channel::<BackgroundTask>();
    let paddle = Paddle::new(
        std::env::var("PADDLE_API_KEY").expect("Paddle API key not found"),
        Paddle::SANDBOX,
    )
    .unwrap();
    let bedrock_client = Arc::new(aws_client::load_bedrock_client().await);

    let state = AppState {
        pinecone_indexes,
        pool: pool.clone(),
        google_client,
        task_queue: tx,
        paddle,
        bedrock_client: bedrock_client.clone(),
    };

    let worker_state = WorkerState {
        pool: pool.clone(),
        pinecone_indexes: state.pinecone_indexes.clone(),
        bedrock_client: bedrock_client.clone(),
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
