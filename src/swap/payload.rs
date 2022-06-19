use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct SwapPayload {
    pub from_coin: String,
    pub to_coin: String,
    pub amount: f64,
    pub blockchain: String
}