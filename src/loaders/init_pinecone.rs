use crate::prelude::*;

pub async fn init_pinecone() -> Index {
    let pinecone_config = PineconeClientConfig {
        api_key: Some(env::var("PINECONE_API_KEY").expect("Pinecone API key not found")),
        ..Default::default()
    };
    let pinecone = pinecone_config
        .client()
        .expect("Failed to create Pinecone instance");

    let index = pinecone
        .index(&env::var("PINECONE_INDEX").expect("Pinecone index not found"))
        .await
        .unwrap();

    index
}
