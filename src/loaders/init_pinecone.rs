use crate::prelude::*;

pub async fn init_pinecone() -> PineconeIndexes {
    let api_key = env::var("PINECONE_API_KEY").expect("Pinecone API key not found");
    let us_east_name = env::var("US_EAST_INDEX").expect("Pinecone index not found");

    //let us_west_name = env::var("US_WEST_INDEX").expect("Pinecone index not found");
    //let eu_west_name = env::var("EU_WEST_INDEX").expect("Pinecone index not found");

    PineconeIndexes::new(api_key, us_east_name)
}
