use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwapData {
    from_coin: String,
    to_coin: String,
    amount: f64
}