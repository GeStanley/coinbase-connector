use crate::coinbase::api::websocket::CoinbaseWebsocketSubscription;
use crate::coinbase::jwt::private_key::CoinbaseCloudApiKey;
use crate::coinbase::jwt::token::CoinbaseJwtToken;

pub struct CoinbaseWebsocketSubscriptionBuilder {
    product_ids: Vec<String>,
    channel: String,
    key: Option<CoinbaseCloudApiKey>,
}

impl CoinbaseWebsocketSubscriptionBuilder {

    pub fn new(product_ids: Vec<String>, channel: String) -> CoinbaseWebsocketSubscriptionBuilder {
        CoinbaseWebsocketSubscriptionBuilder {
            product_ids,
            channel,
            key: None,
        }
    }

    pub fn jwt(&mut self, key: CoinbaseCloudApiKey) -> &mut Self {
        self.key = Some(key);
        self
    }

    pub fn build(&mut self) -> CoinbaseWebsocketSubscription {
        CoinbaseWebsocketSubscription {
            message_type: String::from("subscribe"),
            product_ids: self.product_ids.clone(),
            channel: self.channel.clone(),
            jwt : match CoinbaseJwtToken::new(self.key.clone().unwrap()).sign_websocket() {
                Ok(token) => { token }
                Err(error) => {
                    println!("Error: {:?}", error);
                    panic!("Problem creating jwt token.");
                }
            },
        }
    }
}