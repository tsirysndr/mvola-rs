use crate::types::{
  TransactionDetails, TransactionRequest, TransactionResponse, TransactionStatus,
};
use surf::{Body, Client};

pub struct TransactionService {
  client: Client,
}

impl TransactionService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub async fn get_transaction(&self, id: &str) -> Result<TransactionDetails, surf::Error> {
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
