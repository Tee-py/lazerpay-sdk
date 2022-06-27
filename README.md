lazerpay
=====
A Rust SDK For Integrating Lazerpay payments API

## Overview
Lazerpay allows you to integrate [`lazerpay api`](https://docs.lazerpay.finance/) in rust.

To use the sdk in your application, add this to your `Cargo.toml`:

```toml
[dependencies]
lazerpay = "0.1.0"
```

You also need to get api keys(`SECRET_KEY` & `PUBLIC_KEY`) from your lazerpay dashboard and add then to a `.env` file.

The sdk consist of `misc`, `link`, `swap`, `payments` and `transfer` modules. Each module consist of payload and response structs.

## Usage: `misc::Misc`.

This is used to get accepted coins, get rates and get user balance from the lazerpay api.

### Response Structs

These are the response structs as contained in the `misc::response` sub-module:

| Name | Description |
| :--- | :--- |
| `GetRateResponse` | Response for `get_rate` |
| `BalanceData` | Anatomy of User Balance Data |

#### Example: 

```rust
use std::env;

use dotenv::dotenv;

use lazerpay::{config::ApiConfig, error::Error};
use lazerpay::{misc::*};
use reqwest::Client;

type ResultType = Result<(), Error>;

#[tokio::main]
async fn main() -> ResultType {

    dotenv().ok();
    // You can replace the `SECRET_KEY` & `PUBLIC_KEY` with the name in your .env file.
    let secret_key = env::var("SECRET_KEY").unwrap(); 
    let public_key = env::var("PUBLIC_KEY").unwrap();

    let config = ApiConfig {
        secret_key,
        public_key,
    };

    let client = Client::new();

    let _misc_client = Misc::new(&config, &client);
    let _accepted_coins = _misc_client.get_accepted_coins().await?;
    let _balance = _misc_client.get_balance("USDT").await?;
    let _rate = _misc_client.get_rate("USDT", "USD").await?;

    Ok(());
}
```

## Usage: `link::PaymentLink`
This module is used for creation and payment links management.

### Payload Structs

These are the payload structs contained in the `link::payload` sub-module:

| Name | Description |
| :--- | :--- |
| `CreatePaymentLink` | Payload for payment link creation. |
| `UpdatePaymentLink` | Payload for Updating payment link. |

### Response Structs

These are the response structs contained in the `link::response` sub-module:

| Name | Description |
| :--- | :--- |
| `LinkData` | Anatomy of A PaymentLink Data. |
| `PaymentLinksResponse` | Fetch All Links Response Struct |

```rust
use std::env;

use dotenv::dotenv;

use lazerpay::{config::ApiConfig, error::Error};
use lazerpay::{link::{payload::*, PaymentLink}};
use reqwest::Client;

type ResultType = Result<(), Error>;

#[tokio::main]
async fn main() -> ResultType {

    dotenv().ok();
    // You can replace the `SECRET_KEY` & `PUBLIC_KEY` with the name in your .env file.
    let secret_key = env::var("SECRET_KEY").unwrap(); 
    let public_key = env::var("PUBLIC_KEY").unwrap();

    let config = ApiConfig {
        secret_key,
        public_key,
    };

    let client = Client::new();

    let link_client = PaymentLink::new(&config, &client);

    let _all_links = link_client.fetch_all().await?;
    let create_data = CreatePaymentLink {
        title: "Test".to_string(),
        description: "Testing My Rust SDK.".to_string(),
        amount: 40.0,
        typ: "standard".to_string(),
        currency: "USD".to_string(),
        logo: None,
        redirect_url: None,
    };

    let update_data = UpdatePaymentLink {
        status: "inactive".to_string(),
    };

    // Create Payment Link
    let _create_resp = link_client.create(&create_data).await?;
    // Fetch Payment Link
    let _link = link_client.fetch(&_create_resp.data.id).await?;
    // Update Payment Link
    let _updated = link_client.update(&_link.data.id, &_dat2).await?;
    Ok(())
}
```

## Usage: `payments::Payment`
This module is used for creation and payment links management.

### Payload Structs

These are the payload structs contained in the `payments::payload` sub-module:

| Name | Description |
| :--- | :--- |
| `InitializePayment` | Payload for initializing payment transaction. |


### Response Structs

These are the response structs contained in the `link::response` sub-module:

| Name | Description |
| :--- | :--- |
| `PaymentData` | Payment Information |
| `VerifyPaymentData` | Payment Verification Response Struct |

```rust
use std::env;

use dotenv::dotenv;

use lazerpay::{config::ApiConfig, error::Error};
use lazerpay::payments::{payload::*, Payment};
use lazerpay::utils::*;
use reqwest::Client;

type ResultType = Result<(), Error>;

#[tokio::main]
async fn main() -> ResultType {

    dotenv().ok();
    // You can replace the `SECRET_KEY` & `PUBLIC_KEY` with the name in your .env file.
    let secret_key = env::var("SECRET_KEY").unwrap(); 
    let public_key = env::var("PUBLIC_KEY").unwrap();

    let config = ApiConfig {
        secret_key,
        public_key,
    };

    let client = Client::new();

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
    // Initialize Payment
    let _init_resp = _payment_client.initialize(&_dat).await?;
    // Verify Payment
    let _ver_data = _payment_client.verify(&_dat.reference).await?;
    Ok(())
}
```

For sample Codes on Usage For other Modules check the [`tests`](https://github.com/Tee-py/lazerpay-sdk/tree/main/tests) folder.




### License

This project is licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)
