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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[sqlx(default)]
    pub record_count: Option<i64>,
}

//INDEX
#[derive(Deserialize, Serialize)]
pub struct NamespaceStats {
    pub record_count: u32,
    //pub size: String,
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

//USAGE
#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct UsageLogsResponse {
    pub usage_date: NaiveDate,
    pub credits_used: i32,
}

//USER PORTAL
#[derive(Serialize)]
pub struct PortalUrlBody {
    pub url: String,
}

//TRANSACTIONS
#[derive(Serialize, sqlx::FromRow)]
pub struct Transaction {
    pub plan_name: String,
    pub credits_purchased: i32,
    pub amount_paid: i32,
    pub status: String,
    pub created_at: NaiveDateTime,
}

//USER CREDITS
#[derive(Serialize, sqlx::FromRow)]
pub struct UserCredits {
    pub credits: i32,
}

#[derive(Debug)]
pub enum DashboardError {
    Unforseen,
    Unauthorized,
    MissingData,
    DatabaseExists,
    ApiKeyExists,
    ApiKeyCreationLimit,
    MissingSubscription,
}

impl IntoResponse for DashboardError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            DashboardError::Unforseen => {
                (StatusCode::BAD_REQUEST, "Unforseen error. Contact support")
            }
            DashboardError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            DashboardError::MissingData => (StatusCode::BAD_REQUEST, "Missing form data"),
            DashboardError::DatabaseExists => (StatusCode::BAD_REQUEST, "Database already exists"),
            DashboardError::ApiKeyExists => (StatusCode::BAD_REQUEST, "Api key already exists"),
            DashboardError::ApiKeyCreationLimit => (
                StatusCode::TOO_MANY_REQUESTS,
                "Maximum limit of 10 api keys reached",
            ),
            DashboardError::MissingSubscription => (
                StatusCode::UNAUTHORIZED,
                "Appropriate subscription not found",
            ),
        };

        let body = Json(json!({
            "message": error_message,
        }));

        (status, body).into_response()
    }
}
