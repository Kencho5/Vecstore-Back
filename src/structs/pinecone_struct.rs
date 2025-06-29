use crate::prelude::*;

#[derive(Clone)]
pub struct PineconeIndexes {
    api_key: String,
    us_east_name: String,
    //us_west_name: String,
    //eu_west_name: String,
}

impl PineconeIndexes {
    pub fn new(
        api_key: String,
        us_east_name: String,
        //us_west_name: String,
        //eu_west_name: String,
    ) -> Self {
        Self {
            api_key,
            us_east_name,
            //us_west_name,
            //eu_west_name,
        }
    }

    pub async fn get_index_by_region(&self, region: &str) -> Option<Index> {
        let index_name = match region {
            "us-east" => Some(&self.us_east_name),
            //"us-west" => Some(&self.us_west_name),
            //"eu-west" => Some(&self.eu_west_name),
            _ => {
                eprintln!("Unknown region: {}", region);
                return None;
            }
        };

        if let Some(name) = index_name {
            let pinecone_config = PineconeClientConfig {
                api_key: Some(self.api_key.clone()),
                ..Default::default()
            };
            let pinecone = pinecone_config.client().ok()?;
            pinecone.index(name).await.ok()
        } else {
            eprintln!("Index not configured for region: {}", region);
            None
        }
    }
}
