mod common;

use common::{set_up, TestResult};
use Lazerpay::{transfer::{payload::*, CryptoTransfer}};

#[tokio::test]
async fn test() -> TestResult {
    let (client, config) = set_up();

    let transfer_client = CryptoTransfer::new(&config, &client);

    let payload = Transfer {
        amount: 1.0,
        recipient: "0xF65330dC75e32B20Be62f503a337cD1a072f898f".to_string(),
        coin: "USDT".to_string(),
        blockchain: "Binance Smart Chain".to_string()
    }; 
    let _res = transfer_client.transfer(&payload).await?;
    Ok(())
}