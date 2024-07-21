use crate::coinbase::api::websocket::CoinbaseWebsocketSubscription;
use crate::coinbase::jwt::private_key::CoinbaseCloudApiKey;
use crate::coinbase::jwt::token::CoinbaseJwtToken;

pub struct CoinbaseWebsocketSubscriptionBuilder {
    product_ids: Vec<String>,
    channel: Option<String>,
    key: CoinbaseCloudApiKey,
}

impl CoinbaseWebsocketSubscriptionBuilder {

    pub fn new(key: CoinbaseCloudApiKey) -> CoinbaseWebsocketSubscriptionBuilder {
        CoinbaseWebsocketSubscriptionBuilder {
            product_ids: Vec::new(),
            channel: None,
            key,
        }
    }

    pub fn products(&mut self, product_ids: Vec<String>) -> &mut Self {
        self.product_ids = product_ids;
        self
    }

    pub fn channel(&mut self, channel: String) -> &mut Self {
        self.channel = Some(channel);
        self
    }

    pub fn build(&mut self) -> CoinbaseWebsocketSubscription {
        CoinbaseWebsocketSubscription {
            message_type: String::from("subscribe"),
            product_ids: self.product_ids.clone(),
            channel: self.channel.clone().unwrap(),
            jwt : match CoinbaseJwtToken::new(self.key.clone()).sign_websocket() {
                Ok(token) => { token }
                Err(error) => {
                    println!("Error: {:?}", error);
                    panic!("Problem creating jwt token.");
                }
            },
        }
    }
}