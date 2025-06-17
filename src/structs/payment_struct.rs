use crate::prelude::*;

// PAYMENT CREATED
#[derive(Deserialize)]
pub struct PaymentCreatedPayload {
    pub data: SubscriptionData,
}

#[derive(Deserialize)]
pub struct SubscriptionData {
    pub id: String,                     // subscription_id
    pub status: String,                 // status
    pub next_billed_at: Option<String>, // next_billing_date
    pub items: Vec<SubscriptionItem>,   // to extract plan_id and custom_data
}

#[derive(Deserialize)]
pub struct SubscriptionItem {
    pub product: ProductData,
    pub price: PriceData,
}

#[derive(Deserialize)]
pub struct ProductData {
    pub custom_data: Option<CustomData>,
    pub name: String,
}

#[derive(Deserialize)]
pub struct PriceData {
    pub unit_price: UnitPrice,
}

#[derive(Deserialize)]
pub struct CustomData {
    pub email: Option<String>,
}

#[derive(Deserialize)]
pub struct UnitPrice {
    pub amount: String,
}

pub enum PaymentError {
    Unforseen,
}

impl IntoResponse for PaymentError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            PaymentError::Unforseen => {
                (StatusCode::BAD_REQUEST, "Unforseen error. Contact support")
            }
        };

        let body = Json(json!({
            "message": error_message,
        }));

        (status, body).into_response()
    }
}
