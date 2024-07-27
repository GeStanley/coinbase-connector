use crate::coinbase::jwt::private_key::{CoinbaseCloudApiKey, CoinbaseCloudApiParser};
use crate::coinbase::jwt::token::CoinbaseJwtToken;

pub trait GetRequest {

    fn get_url(&self) -> String;

    fn get_jwt_token(&self) -> String;

}

pub struct GetProducts {
    action: &'static str,
    endpoint: &'static str,
    base_url: &'static str,
    key: CoinbaseCloudApiKey,
}

impl GetProducts {
    pub fn new() -> Self {
        GetProducts {
            action: "GET ",
            endpoint: "products",
            base_url: "api.coinbase.com/api/v3/brokerage/",
            key: CoinbaseCloudApiParser::default().parse(),
        }
    }
}

impl GetRequest for GetProducts {
    fn get_url(&self) -> String {
        "https://".to_owned() + self.base_url + self.endpoint
    }

    fn get_jwt_token(&self) -> String {
        match CoinbaseJwtToken::new(self.key.clone()).sign_http(self.action.to_owned() + self.base_url + self.endpoint) {
            Ok(token) => { token }
            Err(error) => {
                println!("Error: {}", error);
                panic!("Problem creating jwt token.");
            }
        }
    }
}