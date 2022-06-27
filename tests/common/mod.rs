use std::env;

use dotenv::dotenv;
use lazerpay::{config::ApiConfig, error::Error};
use reqwest::Client;

pub type TestResult = Result<(), Error>;

pub fn set_up() -> (Client, ApiConfig) {
    dotenv().ok();
    let secret_key = env::var("SECRET_KEY").unwrap();
    let public_key = env::var("PUBLIC_KEY").unwrap();
    let base_url = env::var("BASE_URL").unwrap();

    let config = ApiConfig {
        secret_key,
        public_key,
        base_url,
    };
    let client = Client::new();

    return (client, config);
}
