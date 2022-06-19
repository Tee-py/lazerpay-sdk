use crate::{
    config::ApiConfig,
    error::Error, response::ApiResponse,
};
use reqwest::blocking::Client;
use reqwest::StatusCode;

use self::{payload::SwapPayload, response::SwapData};

pub mod payload;
mod response;

pub struct CryptoSwap {
    pub api_config: ApiConfig,
    pub api_client: Client,
}

impl CryptoSwap {
    pub fn swap(&self, payload: &SwapPayload) -> Result<(), Error> {
        let url = format!("{}/swap/crypto", self.api_config.base_url);
        let resp = self
            .api_client
            .post(url)
            .headers(self.api_config.create_header())
            .json(payload)
            .send()?;
        // println!("{}", resp.text()?);
        match resp.status() {
            StatusCode::OK => Ok(()),
            _ => Err(Error::RequestError(resp.json()?)),
        }
    }

    pub fn amount_out(&self, payload: &SwapPayload) -> Result<ApiResponse<SwapData>, Error> {
        let url = format!("{}/swap/crypto/amount-out", self.api_config.base_url);
        println!("Start --->");
        let resp = self
            .api_client
            .post(url)
            .headers(self.api_config.create_header())
            .json(&payload)
            .send()?;
        match resp.status() {
            StatusCode::OK => Ok(resp.json()?),
            _ => Err(Error::RequestError(resp.json()?)),
        }
    }
}
