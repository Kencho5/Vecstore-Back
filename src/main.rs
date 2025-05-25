mod prelude;
mod register_routes;

use crate::prelude::*;

#[tokio::main]
async fn main() {
    let app = register_routes::create_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
