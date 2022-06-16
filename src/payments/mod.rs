pub mod payload;
mod response;

use payload::*;
use crate::{config::ApiConfig};

pub struct Transaction {
    config: ApiConfig
}

impl Transaction {

    pub fn initiate_transaction(payload: InitializeTransaction) -> Option<String> {
        Some("Hello".to_string())
    }

    pub fn verify_transaction(tx_ref: &str) -> Option<String> {
        Some("Hello".to_string())
    }
}