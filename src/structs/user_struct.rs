use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub email: String,
    pub name: String,
    pub password: Option<String>,
}
