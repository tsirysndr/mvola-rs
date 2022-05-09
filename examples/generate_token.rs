use mvola::{MVola, SANDBOX_URL};
use std::env;

#[tokio::main]
async fn main() {
  let client = MVola::new(SANDBOX_URL);
  let response = client
    .auth
    .generate_token(
      &env::var("CONSUMER_KEY").unwrap(),
      &env::var("CONSUMER_SECRET").unwrap(),
    )
    .await;

  println!("{:#?}", response);
}
