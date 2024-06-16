use actix::Addr;
use actix_codec::Framed;
use actix_http::ws::{Codec, Frame};
use awc::{BoxedSocket, Client};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::to_string;

use crate::_handle_response;
use crate::coinbase::api::websocket::{WebsocketResponse, WebsocketSubscription};
use crate::coinbase::feed::{CoinbaseMarketData, MarketDataSnapshot, MarketDataUpdate};
use crate::coinbase::jwt::token::{sign_http, sign_websocket};

#[derive(Serialize, Deserialize)]
pub struct CoinbaseCloudApi {
    pub name: String,
    principal: String,
    principalType: String,
    publicKey: String,
    pub privateKey: String,
    createTime: String,
    projectId: String,
    nickname: String,
    scopes: Vec<String>,
    allowedIps: Vec<String>,
    keyType: String,
    enabled: bool,
    legacyScopes: Vec<String>,
    createdByUserId: String,
    createdByUserMongoId: String,
}

#[derive(Serialize, Deserialize)]
pub struct CoinbaseCloudApiV2 {
    pub name: String,
    pub privateKey: String,
}

pub fn build_subscribe(product_ids: Vec<String>, channel: String, jwt: String) -> WebsocketSubscription {
    WebsocketSubscription {
        message_type: String::from("subscribe"),
        product_ids,
        channel,
        jwt,
    }
}

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

pub fn get_subscribe_message(key: &CoinbaseCloudApiV2, product: Vec<String>, channel: String) -> String {
    let jwt_token = match sign_websocket(&key) {
        Ok(token) => { token }
        Err(error) => {
            println!("Error: {:?}", error);
            panic!("Problem creating jwt token.");
        }
    };

    let result = to_string(&build_subscribe(product, channel, jwt_token));
    result.unwrap()
}

pub async fn listen(connection: &mut Framed<BoxedSocket, Codec>, order_book: Addr<CoinbaseMarketData>) {
    while let Some(res) = connection.next().await {
        match res {
            Ok(frame) => {
                match frame {
                    Frame::Text(text) => {
                        // println!("Received Text: {:?}", text);
                        let update: WebsocketResponse = serde_json::from_str(std::str::from_utf8(&*text).unwrap()).unwrap();
                        for event in update.events.iter() {
                            match event.event_type.as_ref().map(String::as_ref) {
                                None => {}
                                Some("snapshot") => {
                                    println!("received snapshot sequence number {}", update.sequence_num);
                                    let update = &event.updates;
                                    let result = order_book.send(MarketDataSnapshot {
                                        id: Default::default(),
                                        msg: "".to_string(),
                                        room_id: Default::default(),
                                    }).await;

                                    match result {
                                        Ok(res) => println!("Got result: {:?}", res),
                                        Err(err) => println!("Got error: {}", err),
                                    }
                                }
                                Some("update") => {
                                    println!("received udpate sequence number {}", update.sequence_num);
                                    let update = &event.updates;
                                    let result = order_book.send(MarketDataUpdate {
                                        id: Default::default(),
                                        msg: "".to_string(),
                                        room_id: Default::default(),
                                    }).await;

                                    match result {
                                        Ok(res) => println!("Got result: {:?}", res),
                                        Err(err) => println!("Got error: {}", err),
                                    }
                                }
                                _ => {}
                            }
                        }
                        println!("serialized");
                    }
                    Frame::Binary(_) => {}
                    Frame::Continuation(_) => {}
                    Frame::Ping(_) => {}
                    Frame::Pong(_) => {}
                    Frame::Close(_) => {}
                }
            }
            Err(error) => {
                println!("Error: {}", error);
                panic!("Problem receiving a websocket message.");
            }
        };
    };
}

pub async fn send_http_request(key: &CoinbaseCloudApiV2) {
    let jwt_token = match sign_http(&key) {
        Ok(token) => { token }
        Err(error) => {
            println!("Error: {}", error);
            panic!("Problem creating jwt token.");
        }
    };

    println!("{}", jwt_token);

    let req = Client::default().get("https://api.coinbase.com/api/v3/brokerage/accounts")
        .insert_header(("Authorization", format!("Bearer {}", jwt_token)));

    let res = req.send().await;

    // println!("Response: {:?}", res);

    match res {
        Ok(response) => { _handle_response(response).await; }
        Err(_) => {}
    }
}