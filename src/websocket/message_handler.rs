use actix::prelude::*;

pub struct WebsocketMessageHandler {

}

impl Actor for WebsocketMessageHandler {
    type Context = Context<Self>;
    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("market data handler started!");
    }
}

