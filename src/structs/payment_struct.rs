use crate::prelude::*;

// PAYMENT CREATED
#[derive(Deserialize)]
pub struct PaymentCreatedPayload {
    pub data: SubscriptionData,
}

#[derive(Deserialize)]
pub struct SubscriptionData {
    pub id: String,
    pub status: String,
    pub next_billed_at: Option<String>,
    pub items: Vec<SubscriptionItem>,
    pub customer_id: String,
    pub custom_data: Option<CustomData>,
}

#[derive(Deserialize)]
pub struct SubscriptionItem {
    pub product: ProductData,
    pub price: PriceData,
}

#[derive(Deserialize)]
pub struct ProductData {
    pub name: String,
}

#[derive(Deserialize)]
pub struct PriceData {
    pub unit_price: UnitPrice,
    pub description: String,
    pub custom_data: CustomPriceData,
}

#[derive(Deserialize)]
pub struct CustomData {
    pub user_id: Option<i32>,
    pub user_email: Option<String>,
}

#[derive(Deserialize)]
pub struct CustomPriceData {
    pub limit: String,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct SubscriptionCanceledPayload {
    pub data: SubscriptionCanceledData,
}

#[derive(Deserialize, Debug)]
pub struct SubscriptionCanceledData {
    pub id: String,
    pub status: String,
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
