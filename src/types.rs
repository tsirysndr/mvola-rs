use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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
pub struct TransactionRequest {
  pub amount: u64,
  pub currency: String,
  pub description_text: String,
  pub request_date: String,
  pub debit_party: Vec<KeyValue>,
  pub credit_party: Vec<KeyValue>,
  pub metadata: Vec<KeyValue>,
  pub requesting_organisation_transaction_reference: String,
  pub original_transaction_reference: String,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionResponse {
  pub status: String,
  pub server_correlation_id: String,
  pub notification_method: String,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionDetails {
  pub amount: u64,
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

#[derive(Serialize, Deserialize)]
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
  pub user_language: String,
  pub user_account_identifier: String,
  pub partner_name: String,
  pub callback_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct KeyValue {
  pub key: String,
  pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Fee {
  pub fee_amount: u16,
}
