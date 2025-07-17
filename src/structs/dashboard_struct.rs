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

#[derive(Deserialize, Serialize)]
pub struct GetDbDocumentsPayload {
    pub name: String,
    pub page: i32,
}

#[derive(Deserialize, Serialize)]
pub struct DocumentsPayload {
    pub data: String,
    pub db_type: String,
    pub name: String,
    pub search_type: String,
    pub region: String,
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

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct DatabaseDocument {
    pub vector_id: String,
    pub content: Option<String>,
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[sqlx(default)]
    pub score: Option<String>,
}

//DELETE DOCUMENT
#[derive(Deserialize, Serialize)]
pub struct DeleteDocumentPayload {
    pub name: String,
    pub document_id: String,
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

//UPLOAD FILES
#[derive(Deserialize, Serialize)]
pub struct UploadFilesPayload {
    pub files: Vec<File>,
    pub files_type: String,
    pub name: String,
    pub region: String,
}

#[derive(Deserialize, Serialize)]
pub struct File {
    pub data: String,
    pub name: String,
}

#[derive(Debug)]
pub enum DashboardError {
    Unforseen,
    //Unauthorized,
    MissingData,
    DatabaseExists,
    ApiKeyExists,
    ApiKeyCreationLimit,
    NotFound,
    NoPaymentMethods,
}

impl IntoResponse for DashboardError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            DashboardError::Unforseen => {
                (StatusCode::BAD_REQUEST, "Unforseen error. Contact support")
            }
            //DashboardError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            DashboardError::MissingData => (StatusCode::BAD_REQUEST, "Missing form data"),
            DashboardError::DatabaseExists => (StatusCode::BAD_REQUEST, "Database already exists"),
            DashboardError::ApiKeyExists => (StatusCode::BAD_REQUEST, "Api key already exists"),
            DashboardError::ApiKeyCreationLimit => (
                StatusCode::TOO_MANY_REQUESTS,
                "Maximum limit of 10 api keys reached",
            ),
            DashboardError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            DashboardError::NoPaymentMethods => (
                StatusCode::NOT_FOUND,
                "No payment methods found. Purchase to continue",
            ),
        };

        let body = Json(json!({
            "message": error_message,
        }));

        (status, body).into_response()
    }
}
