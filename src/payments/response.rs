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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomerInfo {
    id: String,
    customer_name: String,
    customer_email: String,
    customer_phone: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VerifyPaymentData {
    id: String,
    reference: String,
    sender_address: Option<String>,
    recipient_address: String,
    actual_amount: f64,
    amount_paid: Option<f64>,
    amount_paid_fiat: Option<f64>,
    fiat_amount: f64,
    amount_received: Option<f64>,
    amount_received_fiat: Option<f64>,
    coin: String,
    currency: String,
    hash: Option<String>,
    block_number: Option<String>,
    created_at: String,
    updated_at: String,
    #[serde(rename="type")]
    typ: String,
    accept_partial_payment: bool,
    status: String,
    network: String,
    blockchain: String,
    customer: CustomerInfo,
    fee_in_crypto: f64
}