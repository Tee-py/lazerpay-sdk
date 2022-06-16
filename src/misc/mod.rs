mod response;

use reqwest::blocking::Client;
use response::*;

use crate::{
    config::ApiConfig,
    response::{ApiResponse, CoinData},
};

pub struct Misc {
    pub api_config: ApiConfig,
    pub api_client: Client,
}

impl Misc {
    pub fn get_accepted_coins(&self) -> Result<ApiResponse<Vec<CoinData>>, reqwest::Error> {
        let route = format!("{}/coins", self.api_config.base_url);
        let resp = self
            .api_client
            .get(route)
            .headers(self.api_config.headers.clone())
            .send()?;
        // let mut res = match resp {
        //     Ok(res) => res,
        //     Err(e) => return Err(e),
        // };
        //let res: ApiResponse<Vec<CoinData>> = serde_json::from_str(&resp.text()?).unwrap();
        return Ok(resp.json()?);
    }

    pub fn get_rate(&self, coin: &str, currency: &str) -> Result<GetRateResponse, reqwest::Error> {
        let route = format!(
            "{}/rate?coin={}&currency={}",
            self.api_config.base_url, coin, currency
        );
        let resp = self
            .api_client
            .get(route)
            .headers(self.api_config.headers.clone())
            .send()?;
        Ok(resp.json()?)
        // let res: GetRateResponse = serde_json::from_str(resp.text()?);
        // return res;
    }

    pub fn get_balance(&self, coin: &str) -> Result<ApiResponse<BalanceData>, reqwest::Error> {
        let route = format!("{}/wallet/balance?coin={}", self.api_config.base_url, coin);
        let resp = self
            .api_client
            .get(route)
            .headers(self.api_config.headers.clone())
            .send()?;
        // 
        Ok(resp.json()?)
    }
}
