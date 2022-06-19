use crate::{config::ApiConfig, error::Error};

use self::payload::Transfer;

pub mod payload;
mod response;

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Resp {
    pub message: String,
    pub status: String,
    #[serde(rename = "statusCode")]
    pub status_code: i16,
}

pub struct CryptoTransfer {
    pub api_client: reqwest::blocking::Client,
    pub api_config: ApiConfig,
}

impl CryptoTransfer {

    pub fn transfer(&self, payload: &Transfer) -> Result<Resp, Error> {
        let url = format!("{}/transfer", self.api_config.base_url);
        let resp = self
            .api_client
            .post(url)
            .headers(self.api_config.create_header())
            .json(payload)
            .send()?;
        match resp.status() {
            StatusCode::OK => Ok(resp.json()?),
            _ => Err(Error::RequestError(resp.json()?)),
        }
    }
}
