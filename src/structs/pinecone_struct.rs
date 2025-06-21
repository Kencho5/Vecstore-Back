use crate::prelude::*;

pub struct PineconeIndexes {
    pub image_us_east: Arc<Mutex<Index>>,
    //pub image_us_west: Index,
    //pub image_eu: Index,
    pub text_us_east: Arc<Mutex<Index>>,
    //pub text_us_west: Index,
    //pub text_eu: Index,
}
