use actix::{Context, Handler, Message};

use crate::marketdata::order_book::Book;
use crate::websocket::message_handler::WebsocketMessageHandler;

pub struct GetOrderBookRequest {
}

impl Message for GetOrderBookRequest {
    type Result = Result<GetOrderBookResponse, ()>;
}

pub type GetOrderBookResponse = Book;

impl Handler<GetOrderBookRequest> for WebsocketMessageHandler {
    type Result = Result<GetOrderBookResponse, ()>;

    fn handle(&mut self, _req: GetOrderBookRequest, _ctx: &mut Context<Self>) -> Self::Result {
        Ok(self.get_order_book())
    }
}