use std::error::Error;
use std::fs;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use jwtk::ecdsa::EcdsaPrivateKey;
use jwtk::HeaderAndClaims;
use rand::RngCore;
use serde_json::{Map, Value};
use crate::coinbase::coinbase_api::CoinbaseCloudApiV2;

#[non_exhaustive]
pub struct JwtSignatureConfig {
    key_name: String,
    key_secret: String,
    typ: String,
    alg: String,
    kid: String,
    nonce: String,
    sub: String,
    iss: String,
    nbf: Duration,
    exp: Duration,
    aud: Vec<String>,
    uri: String,
}

pub fn create_api_key() -> CoinbaseCloudApiV2 {
    let file_path = "private/coinbase_cloud_api_key.json";
    println!("Reading file {}", file_path);
    // dbg!(&args);
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    match serde_json::from_str(&*contents) {
        Ok(key) => { key }
        Err(error) => {
            println!("Error: {}", error);
            panic!("Problem parsing api key.");
        }
    }
}

pub fn sign_http(key: &CoinbaseCloudApiV2) -> Result<String, impl Error> {
    let encoding_key = EcdsaPrivateKey::from_pem(key.privateKey.as_bytes()).unwrap();

    // let encoding_key = &EncodingKey::from_ec_pem(key_secret_bytes).unwrap();
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let uri = "GET api.coinbase.com/api/v3/brokerage/accounts";

    let mut header_and_claims = HeaderAndClaims::new_dynamic();
    header_and_claims.set_sub(key.name.to_string())
        .set_iss("coinbase-cloud".to_string())
        .set_nbf_from_now(Duration::from_secs(0))
        .set_exp_from_now(Duration::from_secs(120))
        .set_auds(vec!["retail_rest_api_proxy".to_string()])
        .insert("uri", uri)
        .set_kid(key.name.to_string());

    let mut headers_map = Map::new();
    let mut bytes = [0; 16];
    rand::thread_rng().fill_bytes(&mut bytes);
    headers_map.insert("nonce".to_string(), Value::from(hex::encode(&bytes)));
    headers_map.insert("typ".to_string(), Value::from("JWT".to_string()));
    header_and_claims.header_mut().extra = headers_map;

    jwtk::sign(&mut header_and_claims, &encoding_key)
}

pub fn sign_websocket(key: &CoinbaseCloudApiV2) -> Result<String, ()> {
    let encoding_key = EcdsaPrivateKey::from_pem(key.privateKey.as_bytes()).unwrap();

    // let encoding_key = &EncodingKey::from_ec_pem(key_secret_bytes).unwrap();
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let uri = "GET api.coinbase.com/api/v3/brokerage/accounts";

    let mut header_and_claims = HeaderAndClaims::new_dynamic();
    header_and_claims.set_sub(key.name.to_string())
        .set_iss("coinbase-cloud".to_string())
        .set_nbf_from_now(Duration::from_secs(0))
        .set_exp_from_now(Duration::from_secs(120))
        .set_auds(vec!["retail_rest_api_proxy".to_string()])
        .set_kid(key.name.to_string());

    let mut headers_map = Map::new();
    let mut bytes = [0; 16];
    rand::thread_rng().fill_bytes(&mut bytes);
    headers_map.insert("nonce".to_string(), Value::from(hex::encode(&bytes)));
    headers_map.insert("typ".to_string(), Value::from("JWT".to_string()));
    header_and_claims.header_mut().extra = headers_map;

    match jwtk::sign(&mut header_and_claims, &encoding_key) {
        Ok(token) => { Ok(token) }
        Err(_) => { Err(()) }
    }
}