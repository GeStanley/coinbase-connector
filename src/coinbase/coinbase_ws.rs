use std::time::Instant;

use actix::{Actor, Addr, AsyncContext, Context};
use actix_codec::Framed;
use actix_http::ws::Codec;
use awc::BoxedSocket;
use futures_util::SinkExt;
use uuid::Uuid;

use crate::coinbase::coinbase_api::CoinbaseCloudApiV2;
use crate::coinbase::feed::CoinbaseMarketData;
use crate::coinbase::handlers::connect::WebsocketConnect;
use crate::coinbase::handlers::subscribe::WebsocketSubscribe;

pub struct CoinbaseConn {
    pub key: CoinbaseCloudApiV2,
    pub id: Uuid,
    pub data: Addr<CoinbaseMarketData>,
    pub hb: Instant,
    pub websocket: Option<Framed<BoxedSocket, Codec>>,
}

impl Actor for CoinbaseConn {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {

        // let connection_future =
        //     Client::builder().max_http_version(awc::http::Version::HTTP_11).finish()
        //         .ws("wss://advanced-trade-ws.coinbase.com")
        //         .max_frame_size(6000_000)
        //         .connect()
        //         .then(|result| async {
        //             match result {
        //                 Ok((res, conn)) => {
        //                     println!("{:?}", res);
        //                     self.websocket = Some(conn);
        //                 }
        //                 Err(error) => {
        //                     println!("Error: {}", error);
        //                     panic!("Problem creating websocket connection.");
        //                 }
        //             }
        //         });



        // ctx.notify(WebsocketConnect {});
        // ctx.notify(WebsocketSubscribe { product: "BTC-USD".to_string(), channel: "level2".to_string() });
        // loop {
        //     let future_msg = self.websocket.next().then(|event| async {
        //         match event {
        //             Some(result) => {
        //                 match result {
        //                     Ok(frame) => {
        //                         self.handle_frame(frame);
        //                     }
        //                     Err(_) => {}
        //                 }
        //             }
        //             None => {}
        //
        //         }
        //     });
        //
        //     let msg_future = future_msg.into_actor(self);
        //     ctx.wait(msg_future);
        // };
    }
}



