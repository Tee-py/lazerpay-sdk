use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentData {
    pub reference: String,
    pub business_name: String,
    pub business_email: String,
    pub business_logo: String,
    pub customer_name: String,
    pub customer_email: String,
    pub address: String,
    pub coin: String,
    pub crypto_amount: f64,
    pub currency: String,
    pub fiat_amount: f64,
    pub fee_in_crypto: f64,
    pub network: String,
    pub accept_partial_payment: bool
}

pub struct VerifyPaymentResponse {
    pub status: bool,
    pub message: String,
    pub api_response: String
}