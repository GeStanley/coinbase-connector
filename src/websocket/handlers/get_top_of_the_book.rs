use actix::{Context, Handler, Message};

use crate::marketdata::order_book::Book;
use crate::websocket::message_handler::WebsocketMessageHandler;

pub struct GetTopOfTheBookRequest {
}

impl Message for GetTopOfTheBookRequest {
    type Result = Result<GetTopOfTheBookResponse, ()>;
}

pub type GetTopOfTheBookResponse = Book;

impl Handler<GetTopOfTheBookRequest> for WebsocketMessageHandler {
    type Result = Result<GetTopOfTheBookResponse, ()>;

    fn handle(&mut self, _req: GetTopOfTheBookRequest, _ctx: &mut Context<Self>) -> Self::Result {
        Ok(self.get_top_of_the_book())
    }
}