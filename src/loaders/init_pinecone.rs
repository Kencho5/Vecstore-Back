use crate::prelude::*;

pub async fn init_pinecone() -> PineconeIndexes {
    let pinecone_config = PineconeClientConfig {
        api_key: Some(env::var("PINECONE_API_KEY").expect("Pinecone API key not found")),
        ..Default::default()
    };
    let pinecone = pinecone_config
        .client()
        .expect("Failed to create Pinecone instance");

    let image_us_east = pinecone
        .index(&env::var("US_EAST_INDEX").expect("Pinecone index not found"))
        .await
        .unwrap();

    PineconeIndexes {
        us_east: Arc::new(Mutex::new(image_us_east)),
    }
}
