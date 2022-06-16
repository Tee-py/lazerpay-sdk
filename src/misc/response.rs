use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct GetRateResponse {
    pub message: String,
    pub status: String,
    #[serde(rename = "statusCode")]
    pub status_code: i16,
    pub rate: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BalanceData {
    pub coin: String,
    pub amount: f64
}