use crate::coinbase::jwt::private_key::{CoinbaseCloudApiKey, CoinbaseCloudApiParser};
use crate::coinbase::jwt::token::CoinbaseJwtToken;

pub trait GetRequest {

    fn get_url(&self) -> String;

    fn get_jwt_token(&self) -> String;

}

pub struct CoinbaseHttpRequest {
    action: &'static str,
    base_url: &'static str,
    key: CoinbaseCloudApiKey,
}

impl CoinbaseHttpRequest {
    pub fn new() -> Self {
        CoinbaseHttpRequest {
            action: "GET ",
            base_url: "api.coinbase.com/api/v3/brokerage/",
            key: CoinbaseCloudApiParser::default().parse(),
        }
    }

    fn create_base_url(&self, endpoint: &str) -> String {
        "https://".to_owned() + self.base_url + endpoint
    }

    fn create_jwt_token(&self, endpoint: &str) -> String {
        match CoinbaseJwtToken::new(self.key.clone()).sign_http(self.action.to_owned() + self.base_url + endpoint) {
            Ok(token) => { token }
            Err(error) => {
                println!("Error: {}", error);
                panic!("Problem creating jwt token.");
            }
        }
    }
}

pub struct GetProducts {
    coinbase_http_request: CoinbaseHttpRequest,
    endpoint: &'static str,
}

impl GetProducts {
    pub fn new() -> Self {
        GetProducts {
            coinbase_http_request: CoinbaseHttpRequest::new(),
            endpoint: "products",
        }
    }
}

impl GetRequest for GetProducts {
    fn get_url(&self) -> String {
        self.coinbase_http_request.create_base_url(self.endpoint)
    }

    fn get_jwt_token(&self) -> String {
        self.coinbase_http_request.create_jwt_token(self.endpoint)
    }
}

pub struct GetListOrders {
    coinbase_http_request: CoinbaseHttpRequest,
    endpoint: &'static str,
}

impl GetListOrders {
    pub fn new() -> Self {
        GetListOrders {
            coinbase_http_request: CoinbaseHttpRequest::new(),
            endpoint: "orders/historical/batch",
        }
    }
}

impl GetRequest for GetListOrders {
    fn get_url(&self) -> String {
        self.coinbase_http_request.create_base_url(self.endpoint)
    }

    fn get_jwt_token(&self) -> String {
        self.coinbase_http_request.create_jwt_token(self.endpoint)
    }
}