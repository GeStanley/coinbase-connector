use std::ops::Deref;
use actix_codec::Framed;
use actix_http::encoding::Decoder;
use actix_http::Payload;
use actix_http::ws::Codec;
use awc::{BoxedSocket, Client, ClientResponse};
use futures_util::SinkExt;
use serde_json::to_string;

use crate::coinbase::api::subscribe::CoinbaseWebsocketSubscriptionBuilder;
use crate::coinbase::jwt::private_key::CoinbaseCloudApiKey;
use crate::coinbase::jwt::token::CoinbaseJwtToken;

pub async fn connect_websocket() -> Framed<BoxedSocket, Codec> {
    let (_res, connection) = match Client::builder().max_http_version(awc::http::Version::HTTP_11).finish()
        .ws("wss://advanced-trade-ws.coinbase.com")
        .max_frame_size(6000_000)
        .connect()
        .await {
        Ok((_resp, connection)) => (_resp, connection),
        Err(error) => {
            println!("Error: {}", error);
            panic!("Problem creating websocket connection.");
        }
    };

    println!("{:?}", _res);

    connection
}


pub fn get_subscribe_message(key: CoinbaseCloudApiKey, product: Vec<String>, channel: String) -> String {
    let subscription = CoinbaseWebsocketSubscriptionBuilder::new(product, channel)
        .jwt(key)
        .build();
    let result = to_string(&subscription);
    result.unwrap()
}

pub async fn send_http_request(key: CoinbaseCloudApiKey) {
    let uri = "api.coinbase.com/api/v3/brokerage/products";
    let action = "GET ".to_owned();

    let jwt_token = match CoinbaseJwtToken::new(key).sign_http(action + uri) {
        Ok(token) => { token }
        Err(error) => {
            println!("Error: {}", error);
            panic!("Problem creating jwt token.");
        }
    };

    println!("{}", jwt_token);

    let req = Client::default().get("https://".to_owned() + uri)
        .insert_header(("Authorization", format!("Bearer {}", jwt_token)));

    let res = req.send().await;

    println!("Response: {:?}", res);
    match res {
        Ok(response) => { _handle_response(response).await; }
        Err(_) => {}
    }
}


async fn _handle_response(mut response: ClientResponse<Decoder<Payload>>) {
    let fn_name = "handle_response";
    println!("{}: Response: {:?}", fn_name, &response);


    match response.body().await {
        Ok(body) => {
            let _foo = body.to_vec();
            println!("==== BODY ====");
            println!("{:?}", String::from_utf8(_foo));
        }
        Err(_err) => {
            println!("error {:?}", _err);
        }
    }
}