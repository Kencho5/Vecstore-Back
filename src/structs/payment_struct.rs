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
    pub billed_at: Option<String>,
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
