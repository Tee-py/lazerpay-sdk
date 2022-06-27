use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferResponse {
    pub message: String,
    pub status: String,
    #[serde(rename = "statusCode")]
    pub status_code: i16,
}