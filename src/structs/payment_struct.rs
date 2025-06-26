use crate::prelude::*;

// PADDLE WEBHOOK STRUCTS
#[derive(Deserialize, Serialize, Debug)]
pub struct PaymentWebhookPayload {
    pub event_type: String,
    pub data: TransactionData,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransactionData {
    pub id: String,
    pub customer_id: String,
    pub status: String,
    pub billed_at: String,
    pub invoice_id: Option<String>,
    pub invoice_number: Option<String>,
    pub custom_data: CustomData,
    pub items: Vec<TransactionItem>,
    pub payments: Vec<Payment>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CustomData {
    pub user_email: String,
    pub user_id: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransactionItem {
    pub price: Price,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Price {
    pub id: String,
    pub name: String,
    pub description: String,
    pub custom_data: PriceCustomData,
    pub unit_price: UnitPrice,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PriceCustomData {
    pub credits: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UnitPrice {
    pub amount: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Payment {
    pub method_details: MethodDetails,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MethodDetails {
    #[serde(rename = "type")]
    pub payment_type: String,
}

//SUBSCRIPTIONS
#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct Subscription {
    pub subscription_id: String,
    pub plan_name: String,
    pub plan_type: String,
    pub price: i32,
    pub status: String,
    pub next_billing_date: NaiveDate,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct SubscriptionPreviewPayload {
    pub subscription_id: String,
}

#[derive(Serialize)]
pub struct SubscriptionPreview {
    pub subscription_id: String,
    pub next_billed_at: DateTime<chrono::Utc>,
    pub amount: i32,
    pub plan_name: String,
}

pub fn get_price_maps() -> HashMap<String, String> {
    let mut obj = HashMap::new();
    //IMAGE
    obj.insert(
        "pri_01jxwh708k66dmnbsy20ehb713".to_string(),
        "pri_01jxwh9st2t9dkya5jw8trnrg5".to_string(),
    );
    //TEXT
    obj.insert(
        "pri_01jy6ah3zbk4fjwxmw19k6nrpz".to_string(),
        "pri_01jy6aj1b4nwjexky1j731zjwz".to_string(),
    );
    //NSFW
    obj.insert(
        "pri_01jy6akbrzmmpzbkddjcejq71h".to_string(),
        "pri_01jy6akv0t0abdb81eq3q47nwc".to_string(),
    );
    obj
}

pub enum PaymentError {
    Unforseen,
    MissingCustomerData,
}

impl IntoResponse for PaymentError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            PaymentError::Unforseen => {
                (StatusCode::BAD_REQUEST, "Unforseen error. Contact support")
            }
            PaymentError::MissingCustomerData => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Missing customer data. Contact support",
            ),
        };

        let body = Json(json!({
            "message": error_message,
        }));

        (status, body).into_response()
    }
}
