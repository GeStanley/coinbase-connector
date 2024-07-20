use std::error::Error;
use std::time::Duration;

use jwtk::ecdsa::EcdsaPrivateKey;
use jwtk::HeaderAndClaims;
use rand::RngCore;
use serde_json::{Map, Value};

use crate::coinbase::jwt::private_key::CoinbaseCloudApiKey;

pub struct CoinbaseJwtToken {
    key: CoinbaseCloudApiKey,
}

impl CoinbaseJwtToken {
    pub fn new(key: CoinbaseCloudApiKey) -> CoinbaseJwtToken {
        CoinbaseJwtToken { key }
    }

    fn create_headers(self) -> HeaderAndClaims<Map<String, Value>> {
        let mut header_and_claims = HeaderAndClaims::new_dynamic();
        header_and_claims.set_sub(self.key.name.to_string())
            .set_iss("coinbase-cloud".to_string())
            .set_nbf_from_now(Duration::from_secs(0))
            .set_exp_from_now(Duration::from_secs(120))
            .set_auds(vec!["retail_rest_api_proxy".to_string()])
            .set_kid(self.key.name.to_string());

        let mut headers_map = Map::new();
        let mut bytes = [0; 16];
        rand::thread_rng().fill_bytes(&mut bytes);
        headers_map.insert("nonce".to_string(), Value::from(hex::encode(&bytes)));
        headers_map.insert("typ".to_string(), Value::from("JWT".to_string()));
        header_and_claims.header_mut().extra = headers_map;
        header_and_claims
    }

    pub fn sign_http(self, uri: String) -> Result<String, impl Error> {
        let encoding_key = EcdsaPrivateKey::from_pem(self.key.private_key.as_bytes()).unwrap();

        let mut header_and_claims = self.create_headers();
        header_and_claims.insert("uri", uri);

        jwtk::sign(&mut header_and_claims, &encoding_key)
    }

    pub fn sign_websocket(self) -> Result<String, ()> {
        let encoding_key = EcdsaPrivateKey::from_pem(self.key.private_key.as_bytes()).unwrap();

        let mut header_and_claims = self.create_headers();

        match jwtk::sign(&mut header_and_claims, &encoding_key) {
            Ok(token) => { Ok(token) }
            Err(_) => { Err(()) }
        }
    }
}

