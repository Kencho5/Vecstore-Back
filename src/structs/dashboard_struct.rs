use crate::prelude::*;

//ADD DB
#[derive(Deserialize, Serialize)]
pub struct AddDbPayload {
    pub db_type: String,
    pub name: String,
    pub region: String,
}

//GET DBS
#[derive(Deserialize, Serialize)]
pub struct GetDbPayload {
    pub name: String,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct Database {
    pub db_type: String,
    pub name: String,
    pub region: String,
    pub requests: i32,
}

//INDEX
#[derive(Deserialize, Serialize)]
pub struct NamespaceStats {
    pub record_count: u32,
    pub size: String,
}

pub enum DashboardError {
    Unforseen,
    MissingDbData,
    DatabaseExists,
}

impl IntoResponse for DashboardError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            DashboardError::Unforseen => {
                (StatusCode::BAD_REQUEST, "Unforseen error. Contact support")
            }
            DashboardError::MissingDbData => {
                (StatusCode::BAD_REQUEST, "Missing database creation data")
            }
            DashboardError::DatabaseExists => (StatusCode::BAD_REQUEST, "Database already exists"),
        };

        let body = Json(json!({
            "message": error_message,
        }));

        (status, body).into_response()
    }
}
