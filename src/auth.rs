use crate::types::{AuthRequest, AuthResponse};
use surf::http::auth::BasicAuth;
use surf::Client;

pub struct AuthService {
  client: Client,
}

impl AuthService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub async fn generate_token(
    &self,
    consumer_key: &str,
    consumer_secret: &str,
  ) -> Result<(), surf::Error> {
    let auth = BasicAuth::new(consumer_key, consumer_secret);
    Ok(())
  }
}
