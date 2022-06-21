use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinkData {
    pub id: String,
    pub reference: String,
    pub title: String,
    pub amount: f64,
    pub currency: String,
    pub redirect_url: String,
    pub logo: String,
    #[serde(rename = "type")]
    pub typ: String,
    pub network: String,
    pub status: String,
    pub payment_url: String,
    pub created_at: String,
    pub updated_at: String
}