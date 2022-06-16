use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Swap {
    pub from_coin: String,
    pub to_coin: String,
    pub amount: f64,
    pub blockchain: String
}