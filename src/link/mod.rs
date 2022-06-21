use reqwest::StatusCode;

use crate::{config::ApiConfig, error::Error, response::ApiResponse};

use self::{payload::{CreatePaymentLink, UpdatePaymentLink}, response::LinkData};

pub mod payload;
mod response;

pub struct PaymentLink {
    pub api_client: reqwest::blocking::Client,
    pub api_config: ApiConfig,
}

impl PaymentLink {
    pub fn fetch_all(&self) -> Result<ApiResponse<Vec<LinkData>>, Error> {
        let url = format!("{}/payment-links", self.api_config.base_url);
        let resp = self
            .api_client
            .get(url)
            .headers(self.api_config.create_header())
            .send()?;
        match resp.status() {
            StatusCode::OK => Ok(resp.json()?),
            _ => Err(Error::RequestError(resp.json()?)),
        }
    }
    pub fn create(&self, payload: &CreatePaymentLink) -> Result<ApiResponse<LinkData>, Error> {
        let url = format!("{}/payment-links", self.api_config.base_url);
        let resp = self
            .api_client
            .post(url)
            .headers(self.api_config.create_header())
            .json(payload)
            .send()?;
        match resp.status() {
            StatusCode::CREATED => Ok(resp.json()?),
            _ => Err(Error::RequestError(resp.json()?)),
        }
    }

    pub fn fetch(&self, reference: &str) -> Result<ApiResponse<LinkData>, Error> {
        let url = format!("{}/payment-links/{}", self.api_config.base_url, reference);
        let resp = self
            .api_client
            .get(url)
            .headers(self.api_config.create_header())
            .send()?;
        match resp.status() {
            StatusCode::OK => Ok(resp.json()?),
            _ => Err(Error::RequestError(resp.json()?)),
        }
    }

    pub fn update(&self, reference: &str, payload: &UpdatePaymentLink) -> Result<ApiResponse<LinkData>, Error> {
        let url = format!("{}/payment-links/{}", self.api_config.base_url, reference);
        let resp = self
            .api_client
            .put(url)
            .json(payload)
            .headers(self.api_config.create_header())
            .send()?;
        match resp.status() {
            StatusCode::OK => Ok(resp.json()?),
            _ => Err(Error::RequestError(resp.json()?)),
        }
    }
}
