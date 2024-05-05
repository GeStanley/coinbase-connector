use std::fmt::Error;
use std::future::{Future};
use std::time::Instant;

use actix::{Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, Context, Handler};
use actix_codec::Framed;
use actix_http::ws::{Codec, Frame};
use actix_web_actors::ws;
use awc::{BoxedSocket, Client};
use bytestring::ByteString;
use futures_util::{SinkExt, StreamExt, try_join};
use futures_util::stream::Next;
use serde_json::to_string;
use tokio::select;
use uuid::Uuid;

use crate::coinbase::api::websocket::WebsocketResponse;
use crate::coinbase::coinbase_api::{build_subscribe, CoinbaseCloudApiV2};
use crate::coinbase::feed::CoinbaseMarketData;
use crate::coinbase::jwt::token::sign_websocket;

struct WebsocketConnect {}

impl actix::prelude::Message for WebsocketConnect {
    type Result = ();
}

pub struct WebsocketSubscribe {
    product: String,
    channel: String,
}

impl actix::prelude::Message for WebsocketSubscribe {
    type Result = ();
}

pub struct CoinbaseConn {
    key: CoinbaseCloudApiV2,
    id: Uuid,
    data: Addr<CoinbaseMarketData>,
    hb: Instant,
    websocket: Option<Framed<BoxedSocket, Codec>>,
}

impl CoinbaseConn {
    pub fn default(key: CoinbaseCloudApiV2, data: Addr<CoinbaseMarketData>) -> CoinbaseConn {
        CoinbaseConn {
            key,
            id: Uuid::new_v4(),
            hb: Instant::now(),
            data,
            websocket: None,
        }
    }
}

impl CoinbaseConn {
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

impl Actor for CoinbaseConn {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.notify(WebsocketConnect {});
        ctx.notify(WebsocketSubscribe { product: "BTC-USD".to_string(), channel: "level2".to_string() });
        let runtime = tokio::runtime::Runtime::new().unwrap();
        loop {
            let result = runtime.block_on(self.websocket.as_mut().unwrap().next());
            if let Some(msg) = result {
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
    type Result = ();

    fn handle(&mut self, _msg: WebsocketConnect, ctx: &mut Self::Context) -> Self::Result {
        let runtime = tokio::runtime::Runtime::new().unwrap();

        let result = runtime.block_on(
            Client::builder().max_http_version(awc::http::Version::HTTP_11).finish()
                .ws("wss://advanced-trade-ws.coinbase.com")
                .max_frame_size(6000_000)
                .connect()
        );

        match result {
            Ok((resp, connection)) => {
                println!("{:?}", resp);
                self.websocket = Some(connection);
            },
            Err(error) => {
                println!("Error: {}", error);
                ctx.stop();
            },
        }

    }
}

impl Handler<WebsocketSubscribe> for CoinbaseConn {
    type Result = ();
    fn handle(&mut self, msg: WebsocketSubscribe, ctx: &mut Self::Context) -> Self::Result {
        let jwt_token = match sign_websocket(&self.key) {
            Ok(token) => { token }
            Err(error) => {
                println!("Error: {}", error);
                panic!("Problem creating jwt token.");
            }
        };

        let result = to_string(&build_subscribe(vec![msg.product], msg.channel, jwt_token));

        let msg = result.unwrap();

        let runtime = tokio::runtime::Runtime::new().unwrap();
        let result = runtime.block_on(self.websocket.as_mut().unwrap().send(ws::Message::Text(ByteString::from(msg))));

        match result {
            Ok(_) => {
                println!("Message sent.");
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
}
