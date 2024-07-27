use std::fs::File;
use std::io::Write;

use actix_codec::Framed;
use actix_http::encoding::Decoder;
use actix_http::Payload;
use actix_http::ws::Codec;
use awc::{BoxedSocket, Client, ClientResponse};
use futures_util::SinkExt;
use serde_json::to_string;
use crate::coinbase::api::http::GetRequest;
use crate::coinbase::api::subscribe::CoinbaseWebsocketSubscriptionBuilder;
use crate::coinbase::jwt::private_key::{CoinbaseCloudApiKey, CoinbaseCloudApiParser};
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
    let subscription = CoinbaseWebsocketSubscriptionBuilder::new(key)
        .products(product)
        .channel(channel)
        .build();
    let result = to_string(&subscription);
    result.unwrap()
}

pub async fn send_http_request(request: &impl GetRequest) {
    let req = Client::default()
        .get(request.get_url())
        .insert_header(("Authorization", format!("Bearer {}", request.get_jwt_token())));

    let res = req.send().await;

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
            let foo = body.to_vec();
            println!("==== BODY ====");
            let mut file = File::create("response.json").unwrap();
            file.write_all(&*foo).unwrap();
            // println!("{:?}", String::from_utf8(_foo));
        }
        Err(_err) => {
            println!("error {:?}", _err);
        }
    }
}