mod common;

use common::{set_up, TestResult};
use lazerpay::{misc::*};



#[tokio::test]
async fn test() -> TestResult {
    let (client, config) = set_up();

    let misc_client = Misc::new(&config, &client);

    let _resp = misc_client.get_accepted_coins().await?;
    let _resp2 = misc_client.get_balance("USDT").await?;
    let _resp3 = misc_client.get_rate("USDT", "USD").await?;
    Ok(())
}


