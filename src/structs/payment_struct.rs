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
}

#[derive(Deserialize)]
pub struct CustomData {
    pub user_id: Option<i32>,
    pub user_email: Option<String>,
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
    pub price: i32,
    pub status: String,
    pub next_billing_date: NaiveDate,
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
