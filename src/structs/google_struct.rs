use crate::prelude::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct GoogleAuthPayload {
    pub token: String,
}
