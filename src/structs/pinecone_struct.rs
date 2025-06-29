use crate::prelude::*;

#[derive(Clone)]
pub struct PineconeIndexes {
    pub us_east: Arc<Mutex<Index>>,
    //pub image_us_west: Index,
    //pub image_eu: Index,
}

impl PineconeIndexes {
    pub fn get_index_by_region(&self, region: &str) -> Option<Arc<Mutex<Index>>> {
        match region {
            "us-east" => Some(self.us_east.clone()),
            _ => {
                eprintln!("Unknown region: {}", region);
                None
            }
        }
    }
}
