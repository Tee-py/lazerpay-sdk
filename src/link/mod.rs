use reqwest::{StatusCode, Client};

use crate::{config::ApiConfig, error::Error, response::ApiResponse};

use self::{
    payload::{CreatePaymentLink, UpdatePaymentLink},
    response::{LinkData, PaymentLinksResponse},
};

pub mod payload;
mod response;

type LinksResult = Result<PaymentLinksResponse<Vec<LinkData>>, Error>;

pub struct PaymentLink {
    pub api_client: Client,
    pub api_config: ApiConfig,
}

impl PaymentLink {
    pub fn new(config: ApiConfig, client: Client) -> Self {
        Self {
            api_client: client,
            api_config: config,
        }
    }
    pub async fn fetch_all(&self) -> LinksResult {
        let url = format!("{}/payment-links", self.api_config.base_url);
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
    pub async fn create(
        &self,
        payload: &CreatePaymentLink,
    ) -> Result<ApiResponse<LinkData>, Error> {
        let url = format!("{}/payment-links", self.api_config.base_url);
        let resp = self
            .api_client
            .post(url)
            .headers(self.api_config.create_header())
            .json(payload)
            .send()
            .await?;
        match resp.status() {
            StatusCode::CREATED => Ok(resp.json().await?),
            _ => Err(Error::RequestError(resp.json().await?)),
        }
    }

    pub async fn fetch(&self, reference: &str) -> Result<ApiResponse<LinkData>, Error> {
        let url = format!("{}/payment-links/{}", self.api_config.base_url, reference);
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

    pub async fn update(
        &self,
        reference: &str,
        payload: &UpdatePaymentLink,
    ) -> Result<ApiResponse<LinkData>, Error> {
        let url = format!("{}/payment-links/{}", self.api_config.base_url, reference);
        let resp = self
            .api_client
            .put(url)
            .json(payload)
            .headers(self.api_config.create_header())
            .send()
            .await?;
        match resp.status() {
            StatusCode::OK => Ok(resp.json().await?),
            _ => Err(Error::RequestError(resp.json().await?)),
        }
    }
}
