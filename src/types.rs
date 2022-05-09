use serde::{Deserialize, Serialize};

pub trait Service {
  fn set_authorization(&mut self, token: &str);
  fn set_options(&mut self, options: Options);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthResponse {
  pub access_token: String,
  pub token_type: String,
  pub expires_in: u64,
  pub scope: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
  pub grant_type: String,
  pub scope: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRequest {
  pub amount: String,
  pub currency: String,
  pub description_text: String,
  pub request_date: String,
  pub debit_party: Vec<KeyValue>,
  pub credit_party: Vec<KeyValue>,
  pub metadata: Vec<KeyValue>,
  pub requesting_organisation_transaction_reference: String,
  pub original_transaction_reference: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResponse {
  pub status: String,
  pub server_correlation_id: String,
  pub notification_method: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDetails {
  pub amount: String,
  pub currency: String,
  pub transaction_reference: String,
  pub transaction_status: String,
  pub creation_date: String,
  pub request_date: String,
  pub debit_party: Vec<KeyValue>,
  pub credit_party: Vec<KeyValue>,
  pub metadata: Vec<KeyValue>,
  pub fees: Vec<Fee>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionStatus {
  pub status: String,
  pub server_correlation_id: String,
  pub notification_method: String,
  pub object_reference: String,
}

#[derive(Serialize, Deserialize)]
pub struct Options {
  pub version: String,
  pub correlation_id: String,
  pub user_language: Option<String>,
  pub user_account_identifier: String,
  pub partner_name: Option<String>,
  pub callback_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyValue {
  pub key: String,
  pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Fee {
  pub fee_amount: String,
}
