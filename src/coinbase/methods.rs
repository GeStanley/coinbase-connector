use std::fmt::Error;
use std::time::Instant;

use actix::{Addr, ResponseActFuture, WrapFuture};
use actix::Actor;
use actix::prelude::*;
use actix_codec::Framed;
use actix_http::ws::{Codec, Frame, ProtocolError};
use actix_web_actors::ws;
use awc::{BoxedSocket, Client};
use awc::ClientResponse;
use bytestring::ByteString;
use futures_util::SinkExt;
use serde_json::to_string;
use uuid::Uuid;

use crate::coinbase::api::websocket::WebsocketResponse;
use crate::coinbase::coinbase_api::{build_subscribe, CoinbaseCloudApiV2};
use crate::coinbase::coinbase_ws::CoinbaseConn;
use crate::coinbase::feed::CoinbaseMarketData;
use crate::coinbase::jwt::token::sign_websocket;

impl CoinbaseConn {
    pub fn start_with_connection(key: CoinbaseCloudApiV2, data: Addr<CoinbaseMarketData>, connection: Framed<BoxedSocket, Codec>) -> Addr<Self> {
        let actor = Self {
            key,
            id: Uuid::new_v4(),
            hb: Instant::now(),
            data,
            websocket: Some(connection),
        };

        actor.start()
    }

    pub fn start_without_connection(key: CoinbaseCloudApiV2, data: Addr<CoinbaseMarketData>) -> Addr<Self> {
        let actor = Self {
            key,
            id: Uuid::new_v4(),
            hb: Instant::now(),
            data,
            websocket: None,
        };

        actor.start()
    }

    pub async fn send(&mut self, message: String) -> Result<(), ProtocolError> {
        if let Some(connection) = self.websocket.as_mut() {
            connection.send(ws::Message::Text(ByteString::from(message))).await
        } else {
            panic!("No websocket connection!")
        }
    }

    pub fn connect(&mut self) -> ResponseActFuture<Self, Result<ClientResponse, Error>> {
        Box::pin(async {
            match Client::builder().max_http_version(awc::http::Version::HTTP_11).finish()
                .ws("wss://advanced-trade-ws.coinbase.com")
                .max_frame_size(6000_000)
                .connect()
                .await {
                Ok(res) => {
                    Ok(res)
                }
                Err(error) => {
                    Err(error)
                }
            }
        }
                     .into_actor(self)
                     .map(|res, act, _ctx| {
                         match res {
                             Ok((res, frame)) => {
                                 act.websocket = Some(frame);
                                 Ok(res)
                             }
                             Err(_) => {
                                 Err(Error)
                             }
                         }
                     }),
        )
    }
    pub fn subscribe(&mut self, product: String, channel: String) -> ResponseFuture<Result<(), ()>> {
        Box::pin(
            async {
                let jwt_token = match sign_websocket(&self.key) {
                    Ok(token) => { token }
                    Err(error) => {
                        println!("Error: {}", error);
                        panic!("Problem creating jwt token.");
                    }
                };
                let message = to_string(&build_subscribe(vec![product], channel, jwt_token)).unwrap();
                match self.send(message).await {
                    Ok(_) => {
                        println!("Message sent.");
                        Ok(())
                    }
                    Err(error) => {
                        println!("Error: {}", error);
                        Err(())
                    }
                }
            })
    }

    fn handle_frame(&self, frame: Frame) {
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
                        }
                        Some("update") => {
                            println!("received udpate sequence number {}", update.sequence_num);
                            let update = &event.updates;
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
}