use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePaymentLink {
    pub title: String,
    pub description: String,
    pub amount: f64,
    #[serde(rename = "type")]
    pub typ: String,
    pub logo: String,
    pub currency: String,
    pub redirect_url: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePaymentLink {
    pub status: String
}