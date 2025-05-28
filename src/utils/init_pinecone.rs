use crate::prelude::*;

pub async fn init_pinecone() -> PineconeClient {
    let pinecone_config = PineconeClientConfig {
        api_key: Some(env::var("PINECONE_API_KEY").expect("Pinecone API key not found")),
        ..Default::default()
    };
    let pinecone = pinecone_config
        .client()
        .expect("Failed to create Pinecone instance");

    pinecone
}
