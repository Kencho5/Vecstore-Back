use crate::prelude::*;

// PAYMENT CREATED
#[derive(Deserialize, Debug)]
pub struct PaymentWebhookPayload {
    pub event_type: String,
    pub data: SubscriptionData,
}

#[derive(Deserialize, Debug)]
pub struct SubscriptionData {
    pub id: String,
    pub customer_id: String,
    pub status: String,
    pub custom_data: CustomData,
    pub items: Vec<SubscriptionItem>,
    pub next_billed_at: String,
}

#[derive(Deserialize, Debug)]
pub struct CustomData {
    pub user_id: i64,
    pub user_email: String,
}

#[derive(Deserialize, Debug)]
pub struct SubscriptionItem {
    pub price: Price,
}

#[derive(Deserialize, Debug)]
pub struct Price {
    pub name: String,
    pub description: String,
    pub custom_data: PriceCustomData,
    pub unit_price: UnitPrice,
}

#[derive(Deserialize, Debug)]
pub struct PriceCustomData {
    pub db_type: String,
    pub limit: String,
}

#[derive(Deserialize, Debug)]
pub struct UnitPrice {
    pub amount: String,
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
