use actix::Handler;

use crate::coinbase::coinbase_ws::CoinbaseConn;

pub struct WebsocketSubscribe {
    pub product: String,
    pub channel: String,
}

impl actix::prelude::Message for WebsocketSubscribe {
    type Result = ();
}

impl Handler<WebsocketSubscribe> for CoinbaseConn {
    type Result = ();
    fn handle(&mut self, msg: WebsocketSubscribe, ctx: &mut Self::Context) -> Self::Result {
        self.subscribe(msg.product, msg.channel);
    }
}