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
        Self {
            auth: auth::AuthService::new(base_url),
            transaction: transaction::TransactionService::new(base_url),
        }
    }
}
