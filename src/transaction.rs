use crate::types::{
  Options, Service, TransactionDetails, TransactionRequest, TransactionResponse, TransactionStatus,
};
use std::str::FromStr;

use std::time::Duration;
use surf::http::auth::{AuthenticationScheme, Authorization};
use surf::http::{Method, Mime};
use surf::{Client, Config, Url};

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
      user_language: None,
      user_account_identifier: String::from(""),
      partner_name: None,
      callback_url: None,
    };
    Self {
      client,
      base_url: String::from(base_url),
      authorization: None,
      options,
    }
  }

  /// Get the details of a transaction
  /// # Arguments
  /// * `id` - The id of the transaction
  /// # Returns
  /// * `TransactionDetails` - The details of the transaction
  /// # Errors
  /// * `surf::Error` - If the request fails
  /// # Example
  /// ```no_run
  /// #[tokio::main]
  /// async fn main() {
  ///  let mut client = MVola::new(SANDBOX_URL);
  ///  let auth = client
  ///    .auth
  ///    .generate_token(
  ///      &env::var("CONSUMER_KEY").unwrap(),
  ///      &env::var("CONSUMER_SECRET").unwrap(),
  ///    )
  ///    .await;
  ///  client
  ///    .transaction
  ///    .set_authorization(&auth.unwrap().access_token);
  ///  client.transaction.set_options(Options {
  ///    version: String::from("1.0"),
  ///    correlation_id: Uuid::new_v4().to_string(),
  ///    user_language: None,
  ///    user_account_identifier: String::from("msisdn;0343500003"),
  ///    partner_name: None,
  ///    callback_url: None,
  ///  });
  ///  let response = client.transaction.get_transaction("636042511").await;
  ///  println!("{:#?}", response);
  ///}
  /// ```
  pub async fn get_transaction(&self, id: &str) -> Result<TransactionDetails, surf::Error> {
    let path = format!(
      "{}/mvola/mm/transactions/type/merchantpay/1.0.0/{}",
      self.base_url, id
    );
    let url = Url::parse(&path).unwrap();
    let mut req = surf::Request::new(Method::Get, url.clone());
    req.set_header(
      "Authorization",
      self.authorization.as_ref().unwrap().value(),
    );
    req.set_header("Accept", "application/json");
    req.set_header("Version", self.options.version.as_str());
    req.set_header("X-CorrelationID", self.options.correlation_id.as_str());
    req.set_header("Cache-Control", "no-cache");
    req.set_header(
      "UserAccountIdentifier",
      self.options.user_account_identifier.as_str(),
    );

    let res = self.client.recv_json(req).await?;
    Ok(res)
  }

  /// Get the status of a transaction
  /// # Arguments
  /// * `server_correlation_id` - ID from client to uniquely identify the request in client side.
  ///
  /// # Returns
  /// * `TransactionStatus` - The object containing the status of the transaction
  /// # Errors
  /// * `surf::Error` - If the request fails
  /// # Example
  /// ```no_run
  /// #[tokio::main]
  /// async fn main() {
  ///  let mut client = MVola::new(SANDBOX_URL);
  ///  let auth = client
  ///    .auth
  ///    .generate_token(
  ///      &env::var("CONSUMER_KEY").unwrap(),
  ///      &env::var("CONSUMER_SECRET").unwrap(),
  ///    )
  ///    .await;
  ///  client
  ///    .transaction
  ///    .set_authorization(&auth.unwrap().access_token);
  ///  client.transaction.set_options(Options {
  ///    version: String::from("1.0"),
  ///    correlation_id: Uuid::new_v4().to_string(),
  ///    user_language: Some("FR".to_string()),
  ///    user_account_identifier: String::from("msisdn;0343500003"),
  ///    partner_name: Some("TestMVola".to_string()),
  ///    callback_url: None,
  ///  });
  ///  let response = client
  ///    .transaction
  ///    .get_transaction_status("2ba1d66a-25cf-4c12-8a6f-4cb01255148e")
  ///    .await;
  ///  println!("{:#?}", response);
  /// }
  /// ```

  pub async fn get_transaction_status(
    &self,
    server_correlation_id: &str,
  ) -> Result<TransactionStatus, surf::Error> {
    let path = format!(
      "{}/mvola/mm/transactions/type/merchantpay/1.0.0/status/{}",
      self.base_url, server_correlation_id
    );
    let url = Url::parse(&path).unwrap();
    let mut req = surf::Request::new(Method::Get, url.clone());
    req.set_header(
      "Authorization",
      self.authorization.as_ref().unwrap().value(),
    );
    req.set_header("Version", self.options.version.as_str());
    req.set_header("X-CorrelationID", self.options.correlation_id.as_str());
    req.set_header(
      "UserLanguage",
      self.options.user_language.as_ref().unwrap().as_str(),
    );
    req.set_header(
      "PartnerName",
      self.options.partner_name.as_ref().unwrap().as_str(),
    );
    req.set_header("Cache-Control", "no-cache");
    req.set_header(
      "UserAccountIdentifier",
      self.options.user_account_identifier.as_str(),
    );

    let res: TransactionStatus = self.client.recv_json(req).await?;
    Ok(res)
  }

  /// Send a transaction
  /// # Arguments
  /// * `tx` - The transaction to send
  /// # Returns
  /// * `TransactionResponse` - The response of the transaction
  /// # Errors
  /// * `surf::Error` - If the request fails
  /// # Example
  /// ```no_run
  /// #[tokio::main]
  /// async fn main() {
  ///  let mut client = MVola::new(SANDBOX_URL);
  ///  let auth = client
  ///    .auth
  ///    .generate_token(
  ///      &env::var("CONSUMER_KEY").unwrap(),
  ///      &env::var("CONSUMER_SECRET").unwrap(),
  ///    )
  ///    .await;
  ///  client
  ///    .transaction
  ///    .set_authorization(&auth.unwrap().access_token);
  ///  client.transaction.set_options(Options {
  ///    version: String::from("1.0"),
  ///    correlation_id: Uuid::new_v4().to_string(),
  ///    user_language: Some("FR".to_string()),
  ///    user_account_identifier: String::from("msisdn;0343500004"),
  ///    partner_name: Some("TestMVola".to_string()),
  ///    callback_url: None,
  ///  });
  ///  let transaction_ref = Uuid::new_v4();
  ///
  ///  let now = SystemTime::now();
  ///  let now: DateTime<Utc> = now.into();
  ///  let now = now.to_rfc3339_opts(SecondsFormat::Millis, true);
  ///
  ///  let tx: TransactionRequest = TransactionRequest {
  ///    amount: String::from("1000"),
  ///    currency: String::from("Ar"),
  ///    description_text: String::from("test"),
  ///    request_date: now.to_string(),
  ///    debit_party: vec![KeyValue {
  ///      key: String::from("msisdn"),
  ///      value: String::from("0343500003"),
  ///    }],
  ///    credit_party: vec![KeyValue {
  ///      key: String::from("msisdn"),
  ///      value: String::from("0343500004"),
  ///    }],
  ///    metadata: vec![
  ///      KeyValue {
  ///        key: String::from("partnerName"),
  ///        value: String::from("TestMVola"),
  ///      },
  ///      KeyValue {
  ///        key: String::from("fc"),
  ///        value: String::from("USD"),
  ///      },
  ///      KeyValue {
  ///        key: String::from("amountFc"),
  ///        value: String::from("1"),
  ///      },
  ///    ],
  ///
  ///    requesting_organisation_transaction_reference: transaction_ref.to_string(),
  ///    original_transaction_reference: transaction_ref.to_string(),
  ///  };
  ///  let response = client.transaction.send_payment(tx).await;
  ///  println!("{:#?}", response);
  /// }
  /// ```
  pub async fn send_payment(
    &self,
    tx: TransactionRequest,
  ) -> Result<TransactionResponse, surf::Error> {
    let path = format!(
      "{}/mvola/mm/transactions/type/merchantpay/1.0.0/",
      self.base_url
    );
    let url = Url::parse(&path).unwrap();
    let mut req = surf::Request::new(Method::Post, url.clone());
    req.set_header(
      "Authorization",
      self.authorization.as_ref().unwrap().value(),
    );
    req.set_header("Accept", "application/json");
    req.set_header("Version", self.options.version.as_str());
    req.set_header("X-CorrelationID", self.options.correlation_id.as_str());
    req.set_header(
      "UserLanguage",
      self.options.user_language.as_ref().unwrap().as_str(),
    );
    req.set_header(
      "PartnerName",
      self.options.partner_name.as_ref().unwrap().as_str(),
    );
    req.set_header("Cache-Control", "no-cache");
    req.set_header(
      "UserAccountIdentifier",
      self.options.user_account_identifier.as_str(),
    );

    if self.options.callback_url != None {
      req.set_header(
        "X-Callback-URL",
        self.options.callback_url.as_ref().unwrap(),
      );
    }

    req.set_content_type(Mime::from_str("application/json").unwrap());
    req.body_json(&tx).unwrap();
    let res: TransactionResponse = self.client.recv_json(req).await?;
    Ok(res)
  }
}

#[cfg(test)]
mod tests {
  use crate::transaction::TransactionService;
  use crate::types::KeyValue;
  use crate::types::Options;
  use crate::types::Service;
  use crate::types::TransactionRequest;
  use chrono::{DateTime, SecondsFormat, Utc};
  use mockito::{mock, SERVER_URL};
  use std::time::SystemTime;
  use uuid::Uuid;

  #[tokio::test]
  async fn test_send_payment() {
    let _m = mock("POST", "/mvola/mm/transactions/type/merchantpay/1.0.0/")
      .with_status(200)
      .with_header("Content-Type", "application/json")
      .with_body_from_file("tests/fixtures/transaction_response.json")
      .create();

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
    let mut client = TransactionService::new(SERVER_URL);

    client.set_authorization("access token");
    client.set_options(Options {
      version: String::from("1.0"),
      correlation_id: Uuid::new_v4().to_string(),
      user_language: Some("FR".to_string()),
      user_account_identifier: String::from("msisdn;0343500003"),
      partner_name: Some("TestMVola".to_string()),
      callback_url: None,
    });

    let response = client.send_payment(tx).await.unwrap();
    assert_eq!(response.status, "pending");
    assert_eq!(
      response.server_correlation_id,
      "a6b5569b-6181-4fc9-bee3-b9f928dd7ae3"
    );
    assert_eq!(response.notification_method, "polling");
  }

  #[tokio::test]
  async fn test_get_status() {
    let _m = mock(
      "GET",
      "/mvola/mm/transactions/type/merchantpay/1.0.0/status/05AB2C4F-E0E6-42AD-8FA4-9807BDF348BE",
    )
    .with_status(200)
    .with_header("Content-Type", "application/json")
    .with_body_from_file("tests/fixtures/transaction_status.json")
    .create();

    let mut client = TransactionService::new(SERVER_URL);

    client.set_authorization("access token");
    client.set_options(Options {
      version: String::from("1.0"),
      correlation_id: Uuid::new_v4().to_string(),
      user_language: Some("FR".to_string()),
      user_account_identifier: String::from("msisdn;0343500003"),
      partner_name: Some("TestMVola".to_string()),
      callback_url: None,
    });

    let response = client
      .get_transaction_status("05AB2C4F-E0E6-42AD-8FA4-9807BDF348BE")
      .await
      .unwrap();

    assert_eq!(response.status, "completed");
    assert_eq!(
      response.server_correlation_id,
      "2ba1d66a-25cf-4c12-8a6f-4cb01255148e"
    );
  }

  #[tokio::test]
  async fn test_get_transaction() {
    let _m = mock(
      "GET",
      "/mvola/mm/transactions/type/merchantpay/1.0.0/3A5C5E20-B2D9-449F-BBD6-2367A684E9C4",
    )
    .with_status(200)
    .with_header("Content-Type", "application/json")
    .with_body_from_file("tests/fixtures/transaction_details.json")
    .create();

    let mut client = TransactionService::new(SERVER_URL);

    client.set_authorization("access token");
    client.set_options(Options {
      version: String::from("1.0"),
      correlation_id: Uuid::new_v4().to_string(),
      user_language: None,
      user_account_identifier: String::from("msisdn;0343500003"),
      partner_name: None,
      callback_url: None,
    });

    let response = client
      .get_transaction("3A5C5E20-B2D9-449F-BBD6-2367A684E9C4")
      .await
      .unwrap();

    assert_eq!(response.amount, "10000.00");
  }
}
