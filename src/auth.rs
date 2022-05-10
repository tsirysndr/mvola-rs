use crate::types::{AuthRequest, AuthResponse};
use std::str::FromStr;
use std::time::Duration;
use surf::http::auth::BasicAuth;
use surf::http::{Method, Mime};
use surf::{Client, Config, Url};

pub struct AuthService {
  client: Client,
  base_url: String,
}

impl AuthService {
  pub fn new(base_url: &str) -> Self {
    let client = Config::new()
      .set_timeout(Some(Duration::from_secs(5)))
      .try_into()
      .unwrap();
    Self {
      client,
      base_url: String::from(base_url),
    }
  }

  /// Generate a token for the given consumer key and consumer secret.
  ///
  /// The token is valid for one hour.
  ///
  /// The token is then used to set the authorization header for the next request.
  ///
  /// # Arguments
  /// * `consumer_key` - The consumer key
  /// * `consumer_secret` - The consumer secret
  /// # Returns
  /// * `AuthResponse` - The object containing the access token
  /// # Errors
  /// * `surf::Error` - If the request fails
  /// # Example
  /// ```no_run
  ///#[tokio::main]
  /// async fn main() {
  /// let client = MVola::new(SANDBOX_URL);
  /// let response = client
  ///  .auth
  /// .generate_token(
  ///   &env::var("CONSUMER_KEY").unwrap(),
  ///   &env::var("CONSUMER_SECRET").unwrap(),
  /// )
  // .await;
  ///
  ///  println!("{:#?}", response);
  ///}
  /// ```
  pub async fn generate_token(
    &self,
    consumer_key: &str,
    consumer_secret: &str,
  ) -> Result<AuthResponse, surf::Error> {
    let params = AuthRequest {
      grant_type: "client_credentials".to_string(),
      scope: "EXT_INT_MVOLA_SCOPE".to_string(),
    };
    let path = format!("{}/token", &self.base_url);
    let url = Url::parse(&path).unwrap();
    let mut req = surf::Request::new(Method::Post, url.clone());
    req.set_header(
      "Authorization",
      BasicAuth::new(consumer_key, consumer_secret).value(),
    );
    req.set_header("Accept", "application/json");
    req.set_content_type(Mime::from_str("application/x-www-form-urlencoded").unwrap());
    req.body_form(&params);
    let res: AuthResponse = self.client.recv_json(req).await?;
    Ok(res)
  }
}

#[cfg(test)]
mod tests {
  use crate::auth::AuthService;
  use mockito::{mock, SERVER_URL};

  #[tokio::test]
  async fn test_generate_token() {
    let _m = mock("POST", "/token")
      .with_status(200)
      .with_header("Content-Type", "application/json")
      .with_body(
        r#"{
            "access_token": "access_token",
            "expires_in": 3600,
            "token_type": "Bearer",
            "scope": "EXT_INT_MVOLA_SCOPE"
        }"#,
      )
      .create();

    let client = AuthService::new(SERVER_URL);
    let response = client
      .generate_token("consumer_key", "consumer_secret")
      .await
      .unwrap();

    assert_eq!(response.access_token, "access_token");
    assert_eq!(response.expires_in, 3600);
    assert_eq!(response.token_type, "Bearer");
    assert_eq!(response.scope, "EXT_INT_MVOLA_SCOPE");
  }
}
