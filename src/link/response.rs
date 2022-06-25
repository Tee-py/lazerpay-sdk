use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinkData {
    pub id: String,
    pub reference: String,
    pub title: String,
    pub amount: String,
    pub currency: String,
    pub redirect_url: Option<String>,
    pub logo: Option<String>,
    #[serde(rename = "type")]
    pub typ: String,
    pub network: String,
    pub status: String,
    pub payment_url: String,
    pub created_at: String,
    pub updated_at: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentLinksResponse<T> {
    pub status: String,
    pub status_code: i16,
    pub data: T,
    pub count: i16,
    pub current_page: i16,
    pub next_page: Option<i16>,
    pub prev_page: Option<i16>,
    pub last_page: i16
}