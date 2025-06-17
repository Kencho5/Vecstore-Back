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

//API KEYS
#[derive(Deserialize, Serialize)]
pub struct ApiKeyPayload {
    pub key_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteApiKeyPayload {
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct ApiKeysResponse {
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize)]
pub struct ApiKeyResponse {
    pub key: String,
}

//SUBSCRIPTIONS
#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct Subscription {
    pub subscription_id: String,
    pub plan_name: String,
    pub price: i32,
    pub status: String,
    pub next_billing_date: NaiveDate,
}

pub enum DashboardError {
    Unforseen,
    MissingData,
    DatabaseExists,
    ApiKeyExists,
    ApiKeyCreationLimit,
}

impl IntoResponse for DashboardError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            DashboardError::Unforseen => {
                (StatusCode::BAD_REQUEST, "Unforseen error. Contact support")
            }
            DashboardError::MissingData => (StatusCode::BAD_REQUEST, "Missing form data"),
            DashboardError::DatabaseExists => (StatusCode::BAD_REQUEST, "Database already exists"),
            DashboardError::ApiKeyExists => (StatusCode::BAD_REQUEST, "Api key already exists"),
            DashboardError::ApiKeyCreationLimit => (
                StatusCode::TOO_MANY_REQUESTS,
                "Maximum limit of 10 api keys reached",
            ),
        };

        let body = Json(json!({
            "message": error_message,
        }));

        (status, body).into_response()
    }
}
