use std::env;

use dotenv::dotenv;
use lazerpay::{config::ApiConfig, error::Error};
use reqwest::Client;

pub type TestResult = Result<(), Error>;

pub fn set_up() -> (Client, ApiConfig) {
    dotenv().ok();
    let secret_key = env::var("SECRET_KEY").unwrap();
    let public_key = env::var("PUBLIC_KEY").unwrap();

    let config = ApiConfig {
        secret_key,
        public_key,
    };
    let client = Client::new();

    return (client, config);
}
