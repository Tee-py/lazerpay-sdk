mod response;

use reqwest::{blocking::Client, StatusCode};
use response::*;

use crate::{
    config::ApiConfig,
    response::{ApiResponse, CoinData}, error::Error,
};

pub struct Misc {
    pub api_config: ApiConfig,
    pub api_client: Client,
}

impl Misc {
    pub fn get_accepted_coins(&self) -> Result<ApiResponse<Vec<CoinData>>, Error> {
        let route = format!("{}/coins", self.api_config.base_url);
        let resp = self
            .api_client
            .get(route)
            .headers(self.api_config.create_header())
            .send()?;
        match resp.status() {
            StatusCode::OK => Ok(resp.json()?),
            _ => Err(Error::RequestError(resp.json()?)),
        }
    }

    pub fn get_rate(&self, coin: &str, currency: &str) -> Result<GetRateResponse, Error> {
        let route = format!(
            "{}/rate?coin={}&currency={}",
            self.api_config.base_url, coin, currency
        );
        let resp = self
            .api_client
            .get(route)
            .headers(self.api_config.create_header())
            .send()?;
        match resp.status() {
            StatusCode::OK => Ok(resp.json()?),
            _ => Err(Error::RequestError(resp.json()?)),
        }
    }

    pub fn get_balance(&self, coin: &str) -> Result<ApiResponse<BalanceData>, Error> {
        let route = format!("{}/wallet/balance?coin={}", self.api_config.base_url, coin);
        let resp = self
            .api_client
            .get(route)
            .headers(self.api_config.create_header())
            .send()?;
        match resp.status() {
            StatusCode::OK => Ok(resp.json()?),
            _ => Err(Error::RequestError(resp.json()?)),
        }
    }
}
