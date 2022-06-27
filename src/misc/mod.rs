mod response;

use reqwest::{Client, StatusCode};
use response::*;
use crate::constants::BASE_URL;

use crate::{
    config::ApiConfig,
    error::Error,
    response::{ApiResponse, CoinData},
};

/// Performs Miscellaneous operations on the Lazerpay API
/// # Fields
/// * `api_config` - ApiConfig
/// * `api_client` - HTTP Request Client
pub struct Misc<'a> {
    pub api_config: &'a ApiConfig,
    pub api_client: &'a Client,
}

impl<'a> Misc<'a> {
    pub fn new(config: &'a ApiConfig, client: &'a Client) -> Self {
        Self {
            api_client: client,
            api_config: config,
        }
    }
    pub async fn get_accepted_coins(&self) -> Result<ApiResponse<Vec<CoinData>>, Error> {
        let route = format!("{}/coins", BASE_URL);
        let resp = self
            .api_client
            .get(route)
            .headers(self.api_config.create_header())
            .send()
            .await?;
        match resp.status() {
            StatusCode::OK => Ok(resp.json().await?),
            _ => Err(Error::RequestError(resp.json().await?)),
        }
    }

    pub async fn get_rate(&self, coin: &str, currency: &str) -> Result<GetRateResponse, Error> {
        let route = format!(
            "{}/rate?coin={}&currency={}",
            BASE_URL, coin, currency
        );
        let resp = self
            .api_client
            .get(route)
            .headers(self.api_config.create_header())
            .send()
            .await?;
        match resp.status() {
            StatusCode::OK => Ok(resp.json().await?),
            _ => Err(Error::RequestError(resp.json().await?)),
        }
    }

    pub async fn get_balance(&self, coin: &str) -> Result<ApiResponse<BalanceData>, Error> {
        let route = format!("{}/wallet/balance?coin={}", BASE_URL, coin);
        let resp = self
            .api_client
            .get(route)
            .headers(self.api_config.create_header())
            .send()
            .await?;
        match resp.status() {
            StatusCode::OK => Ok(resp.json().await?),
            _ => Err(Error::RequestError(resp.json().await?)),
        }
    }
}
