use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinkData {
    id: String,
    reference: String,
    title: String,
    amount: f64,
    currency: String,
    redirect_url: String,
    logo: String,
    #[serde(rename = "type")]
    typ: String,
    network: String,
    status: String,
    payment_url: String,
    created_at: String,
    updated_at: String
}