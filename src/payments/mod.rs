use reqwest::StatusCode;

use crate::{config::ApiConfig, error::Error, response::ApiResponse};

use self::{
    payload::InitializePayment,
    response::{PaymentData, VerifyPaymentResponse},
};

pub mod payload;
mod response;

pub struct Payment {
    pub api_config: ApiConfig,
    pub api_client: reqwest::blocking::Client,
}

impl Payment {
    pub fn initialize(
        &self,
        payload: &InitializePayment,
    ) -> Result<ApiResponse<PaymentData>, Error> {
        let url = format!("{}/transaction/initialize", self.api_config.base_url);
        let resp = self
            .api_client
            .post(url)
            .json(payload)
            .headers(self.api_config.create_header())
            .send()?;
        match resp.status() {
            StatusCode::OK => Ok(resp.json()?),
            _ => Err(Error::RequestError(resp.json()?)),
        }
    }

    pub fn verify(&self, tx_ref: &str) -> Result<VerifyPaymentResponse, Error> {
        let url = format!("{}/transaction/verify/{}", self.api_config.base_url, tx_ref);
        let resp = self
            .api_client
            .get(url)
            .headers(self.api_config.create_header())
            .send()?;

        match resp.status() {
            StatusCode::OK => {
                let resp_struct = VerifyPaymentResponse {
                    status: true,
                    message: "Payment Retreived".to_string(),
                    api_response: resp.text()?,
                };
                Ok(resp_struct)
            }
            _ => Err(Error::RequestError(resp.json()?))
        }
    }
}
