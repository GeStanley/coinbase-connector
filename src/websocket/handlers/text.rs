use actix::{Context, Handler, Message};
use bytes::Bytes;

use crate::websocket::message_handler::WebsocketMessageHandler;

pub struct ReceivedText {
    pub msg: Bytes,
}

impl Message for ReceivedText {
    type Result = ();
}

impl Handler<ReceivedText> for WebsocketMessageHandler {
    type Result = ();

    fn handle(&mut self, msg: ReceivedText, _ctx: &mut Context<Self>) {
        println!("received coinbase market data message!")
    }
}