use crate::prelude::*;

#[derive(Clone)]
pub struct NeonPools {
    // us_east: PgPool,
    // us_west: PgPool,
    eu: PgPool,
}

impl NeonPools {
    pub fn new(
        // us_east: PgPool,
        // us_west: PgPool,
        eu: PgPool,
    ) -> Self {
        Self {
            // us_east,
            // us_west,
            eu,
        }
    }

    pub fn get_pool_by_region(&self, region: &str) -> Option<&PgPool> {
        match region {
            // "us-east" => Some(&self.us_east),
            // "us-west" => Some(&self.us_west),
            "eu" => Some(&self.eu),
            _ => {
                eprintln!("Unknown Neon region: {}", region);
                None
            }
        }
    }
}
