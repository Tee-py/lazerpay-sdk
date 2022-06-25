mod response;

use reqwest::{Client, StatusCode};
use response::*;

use crate::{
    config::ApiConfig,
    error::Error,
    response::{ApiResponse, CoinData},
};

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
        let route = format!("{}/coins", self.api_config.base_url);
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
            self.api_config.base_url, coin, currency
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
        let route = format!("{}/wallet/balance?coin={}", self.api_config.base_url, coin);
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
