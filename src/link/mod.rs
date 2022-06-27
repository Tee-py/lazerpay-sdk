use reqwest::{StatusCode, Client};

use crate::{config::ApiConfig, error::Error, response::ApiResponse};

use self::{
    payload::{CreatePaymentLink, UpdatePaymentLink},
    response::{LinkData, PaymentLinksResponse},
};

use crate::constants::BASE_URL;

pub mod payload;
mod response;

type LinksResult = Result<PaymentLinksResponse<Vec<LinkData>>, Error>;

pub struct PaymentLink<'a> {
    pub api_client: &'a Client,
    pub api_config: &'a ApiConfig,
}

impl<'a> PaymentLink<'a> {
    pub fn new(config: &'a ApiConfig, client: &'a Client) -> Self {
        Self {
            api_client: client,
            api_config: config,
        }
    }
    pub async fn fetch_all(&self) -> LinksResult {
        let url = format!("{}/payment-links", BASE_URL);
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
        let url = format!("{}/payment-links", BASE_URL);
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
        let url = format!("{}/payment-links/{}", BASE_URL, reference);
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
        let url = format!("{}/payment-links/{}", BASE_URL, reference);
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
