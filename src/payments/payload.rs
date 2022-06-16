use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeTransaction {
    pub reference: String,
    pub customer_name: String,
    pub customer_email: String,
    pub coin: String,
    pub currency: String,
    pub amount: f64,
    pub accept_partial_payment: bool
}