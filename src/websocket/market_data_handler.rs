use actix::prelude::*;
use bytes::Bytes;

pub struct MarketDataHandler {

}

impl Actor for MarketDataHandler {
    type Context = Context<Self>;
    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("market data handler started!");
    }
}

pub struct MessageReceivedRequest {
    pub msg: Bytes,
}

impl Message for MessageReceivedRequest {
    type Result = ();
}

impl Handler<MessageReceivedRequest> for MarketDataHandler {
    type Result = ();

    fn handle(&mut self, msg: MessageReceivedRequest, _ctx: &mut Context<Self>) {
        println!("received coinbase market data message!")
    }
}
