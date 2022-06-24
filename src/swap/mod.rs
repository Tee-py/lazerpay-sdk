use crate::{config::ApiConfig, error::Error, response::ApiResponse};
use reqwest::Client;
use reqwest::StatusCode;

use self::{payload::SwapPayload, response::SwapData};

pub mod payload;
mod response;

pub struct CryptoSwap {
    pub api_config: ApiConfig,
    pub api_client: Client,
}

impl CryptoSwap {
    pub fn new(config: ApiConfig, client: Client) -> Self {
        Self {
            api_config: config,
            api_client: client,
        }
    }

    pub async fn swap(&self, payload: &SwapPayload) -> Result<(), Error> {
        let url = format!("{}/swap/crypto", self.api_config.base_url);
        let resp = self
            .api_client
            .post(url)
            .headers(self.api_config.create_header())
            .json(payload)
            .send()
            .await?;
        // println!("{}", resp.text()?);
        match resp.status() {
            StatusCode::OK => Ok(()),
            _ => Err(Error::RequestError(resp.json().await?)),
        }
    }

    pub async fn amount_out(&self, payload: &SwapPayload) -> Result<ApiResponse<SwapData>, Error> {
        let url = format!("{}/swap/crypto/amount-out", self.api_config.base_url);
        println!("Start --->");
        let resp = self
            .api_client
            .post(url)
            .headers(self.api_config.create_header())
            .json(&payload)
            .send()
            .await?;
        match resp.status() {
            StatusCode::OK => Ok(resp.json().await?),
            _ => Err(Error::RequestError(resp.json().await?)),
        }
    }
}
