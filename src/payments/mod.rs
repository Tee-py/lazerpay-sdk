use reqwest::{Client, StatusCode};

use crate::{config::ApiConfig, error::Error, response::ApiResponse, constants::BASE_URL};
use self::{
    payload::InitializePayment,
    response::{PaymentData, VerifyPaymentData},
};

pub mod payload;
mod response;

pub struct Payment<'a> {
    pub api_config: &'a ApiConfig,
    pub api_client: &'a Client,
}

impl<'a> Payment<'a> {
    pub fn new(config: &'a ApiConfig, client: &'a Client) -> Self {
        Self {
            api_client: client,
            api_config: config,
        }
    }
    pub async fn initialize(
        &self,
        payload: &InitializePayment,
    ) -> Result<ApiResponse<PaymentData>, Error> {
        let url = format!("{}/transaction/initialize", BASE_URL);
        let resp = self
            .api_client
            .post(url)
            .json(payload)
            .headers(self.api_config.create_header())
            .send()
            .await?;
        match resp.status() {
            StatusCode::OK => Ok(resp.json().await?),
            _ => Err(Error::RequestError(resp.json().await?)),
        }
    }

    pub async fn verify(&self, tx_ref: &str) -> Result<ApiResponse<VerifyPaymentData>, Error> {
        let url = format!("{}/transaction/verify/{}", BASE_URL, tx_ref);
        let resp = self
            .api_client
            .get(url)
            .headers(self.api_config.create_header())
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json().await?),
            _ => Err(Error::RequestError(resp.json().await?)),
        }
    }
}
