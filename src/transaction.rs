use crate::types::{
  Options, Service, TransactionDetails, TransactionRequest, TransactionResponse, TransactionStatus,
};
use std::time::Duration;
use surf::http::auth::{AuthenticationScheme, Authorization};
use surf::http::{Method, Mime};
use surf::{Body, Client, Config, Url};

pub struct TransactionService {
  client: Client,
  base_url: String,
  authorization: Option<Authorization>,
  options: Options,
}

impl Service for TransactionService {
  fn set_authorization(&mut self, token: &str) {
    self.authorization = Some(Authorization::new(
      AuthenticationScheme::Bearer,
      String::from(token),
    ));
  }

  fn set_options(&mut self, options: Options) {
    self.options = options;
  }
}

impl TransactionService {
  pub fn new(base_url: &str) -> Self {
    let client = Config::new()
      .set_timeout(Some(Duration::from_secs(5)))
      .try_into()
      .unwrap();
    let options = Options {
      version: String::from("1.0"),
      correlation_id: String::from(""),
      user_language: String::from("FR"),
      user_account_identifier: String::from(""),
      partner_name: String::from(""),
      callback_url: None,
    };
    Self {
      client,
      base_url: String::from(base_url),
      authorization: None,
      options,
    }
  }

  pub async fn get_transaction(&self, id: &str) -> Result<TransactionDetails, surf::Error> {
    let path = format!(
      "{}/mvola/mm/transactions/type/merchantpay/1.0.0/{}",
      self.base_url, id
    );
    let url = Url::parse(&path).unwrap();
    let mut req = surf::Request::new(Method::Get, url.clone());

    let res = self
      .client
      .get(format!(
        "/mvola/mm/transactions/type/merchantpay/1.0.0/{}",
        id
      ))
      .recv_json::<TransactionDetails>()
      .await?;
    Ok(res)
  }

  pub async fn get_transaction_status(
    &self,
    server_correlation_id: &str,
  ) -> Result<TransactionStatus, surf::Error> {
    let res = self
      .client
      .get(format!(
        "/mvola/mm/transactions/type/merchantpay/1.0.0/status/{}",
        server_correlation_id
      ))
      .recv_json::<TransactionStatus>()
      .await?;
    Ok(res)
  }

  pub async fn send_payment(
    &self,
    tx: &TransactionRequest,
  ) -> Result<TransactionResponse, surf::Error> {
    let res = self
      .client
      .post("/mvola/mm/transactions/type/merchantpay/1.0.0/")
      .body(Body::from_json(&tx)?)
      .recv_json::<TransactionResponse>()
      .await?;
    Ok(res)
  }
}
