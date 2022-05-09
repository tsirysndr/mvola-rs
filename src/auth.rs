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
