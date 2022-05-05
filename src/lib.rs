use std::time::Duration;
use surf::http::auth::{AuthenticationScheme, Authorization};
use surf::{Client, Config, Url};

pub mod auth;
pub mod transaction;
pub mod types;

pub const SANDBOX_URL: &str = "https://devapi.mvola.mg";
pub const PRODUCTION_URL: &str = "https://api.mvola.mg";

pub struct MVola {
    pub transaction: transaction::TransactionService,
    pub auth: auth::AuthService,
}

impl MVola {
    pub fn new(base_url: &str) -> Self {
        let client = Config::new()
            .set_base_url(Url::parse(base_url).unwrap())
            .set_timeout(Some(Duration::from_secs(5)))
            .try_into()
            .unwrap();
        Self {
            auth: auth::AuthService::new(&client),
            transaction: transaction::TransactionService::new(&client),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
