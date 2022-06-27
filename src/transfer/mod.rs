use crate::{config::ApiConfig, constants::BASE_URL, error::Error};

use self::{payload::Transfer, response::TransferResponse};

pub mod payload;
mod response;

use reqwest::{Client, StatusCode};

pub struct CryptoTransfer<'a> {
    pub api_client: &'a Client,
    pub api_config: &'a ApiConfig,
}

impl<'a> CryptoTransfer<'a> {
    pub fn new(config: &'a ApiConfig, client: &'a Client) -> Self {
        Self {
            api_client: client,
            api_config: config,
        }
    }
    pub async fn transfer(&self, payload: &Transfer) -> Result<TransferResponse, Error> {
        let url = format!("{}/transfer", BASE_URL);
        let resp = self
            .api_client
            .post(url)
            .headers(self.api_config.create_header())
            .json(payload)
            .send()
            .await?;
        match resp.status() {
            StatusCode::OK => Ok(resp.json().await?),
            _ => Err(Error::RequestError(resp.json().await?)),
        }
    }
}
