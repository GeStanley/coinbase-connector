use std::fmt::Error;
use std::future::{Future, join};
use std::time::Instant;

use actix::{Actor, ActorFutureExt, Addr, AsyncContext, Handler};
use actix::prelude::{Message};
use actix_codec::Framed;
use actix_http::ws::{Codec, Frame};
use actix_web_actors::ws;
use awc::{BoxedSocket, Client};
use bytestring::ByteString;
use futures_util::{SinkExt, StreamExt, try_join};
use serde_json::to_string;
use tokio::select;
use uuid::Uuid;

use crate::coinbase::api::websocket::WebsocketResponse;
use crate::coinbase::coinbase_api::{build_subscribe, CoinbaseCloudApi};
use crate::coinbase::feed::CoinbaseMarketData;
use crate::coinbase::jwt::token::sign_websocket;

struct WebsocketConnect {}

impl Message for WebsocketConnect {
    type Result = ();
}

pub struct CoinbaseConn {
    key: CoinbaseCloudApi,
    id: Uuid,
    data: Addr<CoinbaseMarketData>,
    hb: Instant,
    websocket: Framed<BoxedSocket, Codec>,
}

impl CoinbaseConn {
    pub fn new(key: CoinbaseCloudApi, data: Addr<CoinbaseMarketData>, framed: Framed<BoxedSocket, Codec>) -> CoinbaseConn {
        CoinbaseConn {
            key,
            id: Uuid::new_v4(),
            hb: Instant::now(),
            data,
            websocket: framed,
        }
    }
}

impl CoinbaseConn {

    pub fn subscribe(&mut self, product: String, channel: String) {
        let jwt_token = match sign_websocket(&self.key) {
            Ok(token) => {token}
            Err(error) => {
                println!("Error: {}", error);
                panic!("Problem creating jwt token.");
            }
        };

        let result = to_string(&build_subscribe(vec![product], channel, jwt_token));

        let msg = result.unwrap();
        println!("{}", msg);

        // match self.websocket
        //     .send(Message::Text(ByteString::from(message))) {
        //     Ok(_) => {
        //         println!("Message sent.");
        //     },
        //     Err(error) => {
        //         println!("Error: {}", error);
        //         panic!("Problem sending on websocket connection.");
        //     },
        // };
    }

    fn handle_frame(&self, frame: Frame) {
        match frame {
            Frame::Text(text) => {
                // println!("Received Text: {:?}", text);
                let update : WebsocketResponse = serde_json::from_str(std::str::from_utf8(&*text).unwrap()).unwrap();
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
            Frame::Binary(_) => {

            }
            Frame::Continuation(_) => {

            }
            Frame::Ping(_) => {

            }
            Frame::Pong(_) => {

            }
            Frame::Close(_) => {

            }
        }
    }
}

impl Actor for CoinbaseConn {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {

        let result = ctx.notify(WebsocketConnect {});

        loop {

            let Some(msg) = self.websocket.next() {
                match msg {
                    Ok(frame) => {
                        self.handle_frame(frame);
                    }
                    Err(error) => {
                        println!("Error: {}", error);
                        panic!("Problem receiving a websocket message.");
                    }
                }
            }

        }
    }
}

impl Handler<WebsocketConnect> for CoinbaseConn {
    type Result = Result<Framed<BoxedSocket, Codec>, Error>;
    fn handle(&mut self, _msg: WebsocketConnect, ctx: &mut Self::Context) -> Self::Result {
        let t1  = Client::builder().max_http_version(awc::http::Version::HTTP_11).finish()
            .ws("wss://advanced-trade-ws.coinbase.com")
            .max_frame_size(6000_000)
            .connect();

        try_join!(t1)
    }
}
