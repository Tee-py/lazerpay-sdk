use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transfer {
    pub amount: f64,
    pub recipient: String,
    pub coin: String,
    pub blockchain: String
}