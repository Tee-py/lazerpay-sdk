#[macro_use]
extern crate serde_derive;
mod config;
mod error;
pub mod link;
pub mod misc;
pub mod payments;
mod response;
pub mod swap;
pub mod transfer;

#[cfg(test)]
mod tests {
    use crate::config::ApiConfig;
    use crate::link::{payload::*, PaymentLink};
    use crate::payments::Payment;
    use crate::misc::*;
    use crate::payments::payload::*;
    use crate::response::*;
    use crate::swap::{payload::*, CryptoSwap};
    use crate::transfer::{payload::*, CryptoTransfer};
    use dotenv::dotenv;
    use reqwest::Client;
    use std::env;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn test_payload_structs() {
        let data1 = r#"{"reference":"5152eft78","customer_name":"Test Test","customer_email":"test@email.com","coin":"USDT","currency":"USD","amount":56.8,"accept_partial_payment":false}"#;
        let data2 = r#"{"title":"Test","description":"Test","type":"Test","currency":"USD","amount":56.8,"logo":"Test","redirect_url":"Test"}"#;
        let data3 = r#"{"status": "active"}"#;
        let data4 = r#"{"amount": 40.0, "recipient": "0x2323rb23uri9bg3yu4r", "coin": "ETH", "blockchain": "Ethereum"}"#;
        let data5 =
            r#"{"amount": 40.0, "fromCoin": "USDT", "toCoin": "ETH", "blockchain": "Ethereum"}"#;

        let tx_data: InitializePayment = serde_json::from_str(data1).unwrap();
        let link_data: CreatePaymentLink = serde_json::from_str(data2).unwrap();
        let update_link_data: UpdatePaymentLink = serde_json::from_str(data3).unwrap();
        let transfer_data: Transfer = serde_json::from_str(data4).unwrap();
        let swap_data: SwapPayload = serde_json::from_str(data5).unwrap();

        // Assertions
        assert_eq!(&tx_data.reference, "5152eft78");
        assert_eq!(&tx_data.customer_name, "Test Test");
        assert_eq!(&tx_data.customer_email, "test@email.com");
        assert_eq!(&tx_data.coin, "USDT");
        assert_eq!(&tx_data.currency, "USD");
        assert_eq!(&tx_data.amount, &56.8);
        assert!(!&tx_data.accept_partial_payment);
        assert_eq!(&link_data.title, "Test");
        assert_eq!(&link_data.description, "Test");
        assert_eq!(&link_data.typ, "Test");
        assert_eq!(&link_data.currency, "USD");
        assert_eq!(&link_data.amount, &56.8);
        assert_eq!(&link_data.logo, "Test");
        assert_eq!(&link_data.redirect_url, "Test");
        assert_eq!(&update_link_data.status, "active");
        assert_eq!(&transfer_data.amount, &40.0);
        assert_eq!(&transfer_data.recipient, "0x2323rb23uri9bg3yu4r");
        assert_eq!(&transfer_data.coin, "ETH");
        assert_eq!(&transfer_data.blockchain, "Ethereum");
        assert_eq!(&swap_data.from_coin, "USDT");
        assert_eq!(&swap_data.to_coin, "ETH");
        assert_eq!(&swap_data.amount, &40.0);
        assert_eq!(&swap_data.blockchain, "Ethereum");
    }

    #[test]
    fn test_response_struct() {
        let data = r#"{
            "message": "Retrive accepted coins",
            "data": [
              {
                "id": "56906c2a-b2d7-4efe-92f6-2a079f844a2e",
                "name": "Dai Stablecoin",
                "symbol": "DAI",
                "logo": "https://cryptologos.cc/logos/multi-collateral-dai-dai-logo.png?v=014",
                "address": "0x1AF3F329e8BE154074D8769D1FFa4eE058B1DBc3",
                "network": "mainnet",
                "blockchain": "Binance Smart Chain",
                "status": "active",
                "createdAt": "2021-12-01T16:29:42.518Z",
                "updatedAt": "2021-12-01T16:29:42.518Z"
              },
              {
                "id": "b31db87a-aba0-4409-9521-c5c6611850f7",
                "name": "USD Coin",
                "symbol": "USDC",
                "logo": "https://cryptologos.cc/logos/usd-coin-usdc-logo.png?v=014",
                "address": "0x8AC76a51cc950d9822D68b83fE1Ad97B32Cd580d",
                "network": "mainnet",
                "blockchain": "Binance Smart Chain",
                "status": "active",
                "createdAt": "2021-12-01T16:29:42.518Z",
                "updatedAt": "2021-12-01T16:29:42.518Z"
              },
              {
                "id": "c29c9762-b9a4-4e35-a895-f62ea99a3f58",
                "name": "Binance USD",
                "symbol": "BUSD",
                "logo": "https://cryptologos.cc/logos/binance-usd-busd-logo.png?v=014",
                "address": "0xe9e7CEA3DedcA5984780Bafc599bD69ADd087D56",
                "network": "mainnet",
                "blockchain": "Binance Smart Chain",
                "status": "active",
                "createdAt": "2021-12-01T16:29:42.527Z",
                "updatedAt": "2021-12-01T16:29:42.527Z"
              },
              {
                "id": "e472302a-f639-45d2-8916-8f3408781200",
                "name": "Tether (USDT)",
                "symbol": "USDT",
                "logo": "https://cryptologos.cc/logos/tether-usdt-logo.png?v=014",
                "address": "0x55d398326f99059fF775485246999027B3197955",
                "network": "mainnet",
                "blockchain": "Binance Smart Chain",
                "status": "active",
                "createdAt": "2021-12-01T16:29:42.526Z",
                "updatedAt": "2021-12-01T16:29:42.526Z"
              }
            ],
            "status": "success",
            "statusCode": 200
          }"#;

        let res: ApiResponse<Vec<CoinData>> = serde_json::from_str(data).unwrap();

        println!("Thirstyyyyy--> {}", res.data[0].created_at);
    }

    #[tokio::test]
    async fn test_misc() -> TestResult {
        // Load Env Variables
        dotenv().ok();
        let secret_key = env::var("SECRET_KEY")?;
        let public_key = env::var("PUBLIC_KEY")?;
        let base_url = env::var("BASE_URL").unwrap();
        let config = ApiConfig {
            secret_key,
            public_key,
            base_url,
        };
        let client = Client::new();

        let misc = Misc {
            api_config: config,
            api_client: client,
        };
        let coins = misc.get_accepted_coins().await?;
        let rate = misc.get_rate("ETH", "USD").await?;
        let balance = misc.get_balance("USDT").await?;

        assert_eq!(coins.status_code, 200);
        assert_eq!(rate.status_code, 200);
        assert_eq!(balance.status_code, 200);

        Ok(())
    }

    #[tokio::test]
    async fn test_swap() -> TestResult {
        dotenv().ok();
        let secret_key = env::var("SECRET_KEY")?;
        let public_key = env::var("PUBLIC_KEY")?;
        let base_url = env::var("BASE_URL").unwrap();
        let config = ApiConfig {
            secret_key,
            public_key,
            base_url,
        };
        let client = Client::new();

        let crypto_swap = CryptoSwap {
            api_client: client,
            api_config: config,
        };
        let swap_payload = SwapPayload {
            to_coin: "USDT".to_string(),
            from_coin: "BNB".to_string(),
            amount: 0.1,
            blockchain: "Binance Smart Chain".to_string(),
        };
        let _swap_res = crypto_swap.swap(&swap_payload).await?;
        let _amount_out = crypto_swap.amount_out(&swap_payload).await?;
        Ok(())
    }
    #[tokio::test]
    async fn test_transfer() -> TestResult {
      dotenv().ok();
      let secret_key = env::var("SECRET_KEY")?;
      let public_key = env::var("PUBLIC_KEY")?;
      let base_url = env::var("BASE_URL").unwrap();
      let config = ApiConfig {
          secret_key,
          public_key,
          base_url,
      };
      let client = Client::new();

      let transfer_client = CryptoTransfer {
        api_client: client,
        api_config: config
      };
      let payload = Transfer {
        amount: 100.0,
        recipient: "0x0B4d358D349809037003F96A3593ff9015E89efA".to_string(),
        coin: "USDT".to_string(),
        blockchain: "Binance Smart Chain".to_string()
      }; 
      let _res = transfer_client.transfer(&payload).await?;
      Ok(())
    }

    #[tokio::test]
    async fn test_link() -> TestResult {
      dotenv().ok();
      let secret_key = env::var("SECRET_KEY")?;
      let public_key = env::var("PUBLIC_KEY")?;
      let base_url = env::var("BASE_URL").unwrap();
      let config = ApiConfig {
          secret_key,
          public_key,
          base_url,
      };
      let client = Client::new();

      let link_client = PaymentLink { api_client: client, api_config: config };
      let _all_links = link_client.fetch_all().await?;
      let dat1 = CreatePaymentLink {
        title: "Test".to_string(),
        description: "Test".to_string(),
        amount: 40.0,
        typ: "standard".to_string(),
        currency: "USD".to_string(),
        logo: "https://test.com/logo.png".to_string(),
        redirect_url: "https://test.com/payment-redirect".to_string()
      };
      let _create_resp = link_client.create(&dat1).await?;
      let _link = link_client.fetch(&_create_resp.data.id).await?;
      Ok(())
    }

    #[tokio::test]
    async fn test_payment() -> TestResult {
      dotenv().ok();
      let secret_key = env::var("SECRET_KEY")?;
      let public_key = env::var("PUBLIC_KEY")?;
      let base_url = env::var("BASE_URL").unwrap();
      let config = ApiConfig {
          secret_key,
          public_key,
          base_url,
      };
      let client = Client::new();

      let _payment_client = Payment { api_client: client, api_config: config };
      // Test Initialize Payment
      // Test Verify Payment
      Ok(())
    }
}
