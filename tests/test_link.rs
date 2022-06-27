mod common;

use common::{set_up, TestResult};
use Lazerpay::link::{payload::*, PaymentLink};

#[tokio::test]
async fn test() -> TestResult {
    let (client, config) = set_up();

    let link_client = PaymentLink::new(&config, &client);

    let _all_links = link_client.fetch_all().await?;
    let _dat1 = CreatePaymentLink {
        title: "Test".to_string(),
        description: "Testing My Rust SDK.".to_string(),
        amount: 40.0,
        typ: "standard".to_string(),
        currency: "USD".to_string(),
        logo: None,
        redirect_url: None,
    };
    let _dat2 = UpdatePaymentLink {
        status: "inactive".to_string(),
    };
    let _create_resp = link_client.create(&_dat1).await?;
    let _link = link_client.fetch(&_create_resp.data.id).await?;
    let _updated = link_client.update(&_link.data.id, &_dat2).await?;
    assert_eq!(_updated.data.status, _dat2.status);
    Ok(())
}
