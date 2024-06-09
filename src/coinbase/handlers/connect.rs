use actix::Handler;
use crate::coinbase::coinbase_ws::CoinbaseConn;

pub struct WebsocketConnect {}

impl actix::prelude::Message for WebsocketConnect {
    type Result = ();
}

impl Handler<WebsocketConnect> for CoinbaseConn {
    type Result = ();

    fn handle(&mut self, _msg: WebsocketConnect, ctx: &mut Self::Context) -> Self::Result {

    }
}