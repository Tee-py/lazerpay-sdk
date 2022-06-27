mod common;

use common::{set_up, TestResult};
use lazerpay::swap::{payload::*, CryptoSwap};

#[tokio::test]
async fn test_swap() -> TestResult {
    let (client, config) = set_up();

    let crypto_swap = CryptoSwap::new(&config, &client);
    let swap_payload = SwapPayload {
        to_coin: "USDT".to_string(),
        from_coin: "BNB".to_string(),
        amount: 0.1,
        blockchain: "Binance Smart Chain".to_string(),
    };
    let _swap_res = crypto_swap.swap(&swap_payload);
    let _amount_out = crypto_swap.amount_out(&swap_payload);
    Ok(())
}
