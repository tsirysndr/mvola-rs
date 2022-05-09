use mvola::types::Options;
use mvola::types::Service;
use mvola::{MVola, SANDBOX_URL};
use std::env;
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
  let response = client
    .transaction
    .get_transaction_status("2ba1d66a-25cf-4c12-8a6f-4cb01255148e")
    .await;
  println!("{:#?}", response);
}
