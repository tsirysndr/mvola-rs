<h1>mvola-rs</h1>
<p>
  <a href="https://app.travis-ci.com/github/tsirysndr/mvola-rs" target="_blank">
    <img src="https://app.travis-ci.com/tsirysndr/mvola-rs.svg?branch=master" />
  </a>
  <a href="https://codecov.io/gh/tsirysndr/mvola-rs" target="_blank">
    <img src="https://codecov.io/gh/tsirysndr/mvola-rs/branch/master/graph/badge.svg?token=" />
  </a>
  <a href="LICENSE" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-blue.svg" />
  </a>
  <a href="https://crates.io/crates/mvola" target="_blank">
    <img src="https://img.shields.io/crates/v/mvola.svg" />
  </a>
  <a href="https://docs.rs/mvola" target="_blank">
    <img src="https://docs.rs/mvola/badge.svg" />
  </a>
</p>

[MVola](https://www.mvola.mg/devportal) Rust client library.

## Install
Add the following line to your `Cargo.toml` file:
```toml
[dependencies]
mvola = "0.1"
```

## Usage

```rust
use chrono::{DateTime, SecondsFormat, Utc};
use mvola::types::KeyValue;
use mvola::types::Options;
use mvola::types::Service;
use mvola::types::TransactionRequest;
use mvola::{MVola, SANDBOX_URL};
use std::env;
use std::time::SystemTime;
use uuid::Uuid;

#[tokio::main]
async fn main() {
  let mut client = MVola::new(SANDBOX_URL);
  let auth = client
    .auth
    .generate_token(
      &env::var("CONSUMER_KEY").unwrap(),
      &env::var("CONSUMER_SECRET").unwrap(),
    )
    .await;
  client
    .transaction
    .set_authorization(&auth.unwrap().access_token);
  client.transaction.set_options(Options {
    version: String::from("1.0"),
    correlation_id: Uuid::new_v4().to_string(),
    user_language: Some("FR".to_string()),
    user_account_identifier: String::from("msisdn;0343500003"),
    partner_name: Some("TestMVola".to_string()),
    callback_url: None,
  });
  let transaction_ref = Uuid::new_v4();

  let now = SystemTime::now();
  let now: DateTime<Utc> = now.into();
  let now = now.to_rfc3339_opts(SecondsFormat::Millis, true);

  let tx: TransactionRequest = TransactionRequest {
    amount: String::from("1000"),
    currency: String::from("Ar"),
    description_text: String::from("test"),
    request_date: now.to_string(),
    debit_party: vec![KeyValue {
      key: String::from("msisdn"),
      value: String::from("0343500003"),
    }],
    credit_party: vec![KeyValue {
      key: String::from("msisdn"),
      value: String::from("0343500004"),
    }],
    metadata: vec![
      KeyValue {
        key: String::from("partnerName"),
        value: String::from("TestMVola"),
      },
      KeyValue {
        key: String::from("fc"),
        value: String::from("USD"),
      },
      KeyValue {
        key: String::from("amountFc"),
        value: String::from("1"),
      },
    ],

    requesting_organisation_transaction_reference: transaction_ref.to_string(),
    original_transaction_reference: transaction_ref.to_string(),
  };
  let response = client.transaction.send_payment(tx).await;
  println!("{:#?}", response);
}

```

### Test

```sh 
cargo test
```

## Author

👤 **Tsiry Sandratraina <tsiry.sndr@aol.com>**

* Twitter: [@tsiry_sndr](https://twitter.com/tsiry_sndr)
* Github: [@tsirysndr](https://github.com/tsirysndr)

## Show your support

Give a ⭐️ if this project helped you!
