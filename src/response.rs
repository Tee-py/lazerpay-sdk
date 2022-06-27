use serde::{Deserialize, Serialize};


/// lazerpay Api response structure
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub message: String,
    pub status: String,
    #[serde(rename = "statusCode")]
    /// HTTP status code
    pub status_code: i16,
    /// Response data
    pub data: T,
}

/// Structure of Lazerpay Accepted Coin
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CoinData {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub logo: String,
    pub address: String,
    /// testnet or mainnet
    pub network: String,
    pub blockchain: String,
    pub status: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
