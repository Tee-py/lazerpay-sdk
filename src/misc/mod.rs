mod response;

use reqwest::Client;

use crate::{
    config::ApiConfig,
    response::{ApiResponse, CoinData},
};

pub struct Misc {
    api_config: ApiConfig,
    api_client: Client,
}

impl Misc {

    pub fn get_accepted_coins() -> Option<String> {
        Some("Hello".to_string())
    }

    pub fn get_rate(&self, coin: &str, currency: &str) -> Option<String> {
        Some("Hello".to_string())
    }

    pub fn get_balance(&self, coin: &str) -> Option<String> {
        Some("Yes".to_string())
    }
}
