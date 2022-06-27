mod common;

use common::{set_up, TestResult};
use Lazerpay::payments::{payload::*, Payment};
use Lazerpay::utils::*;

#[tokio::test]
async fn test() -> TestResult {
    let (client, config) = set_up();

    let _payment_client = Payment::new(&config, &client);

    let tx_ref = generate_reference();
    let _dat = InitializePayment {
        reference: tx_ref,
        customer_name: "Test Customer".to_string(),
        customer_email: "customer@lazertest.com".to_string(),
        coin: "USDT".to_string(),
        currency: "USD".to_string(),
        amount: 4.0,
        accept_partial_payment: true,
    };
    // Test Initialize Payment
    let _init_resp = _payment_client.initialize(&_dat).await?;
    // Test Verify Payment
    let _ver_data = _payment_client.verify(&_dat.reference).await?;
    Ok(())
}
