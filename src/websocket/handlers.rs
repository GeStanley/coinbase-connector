use actix::{Handler, Message, ResponseActFuture, ResponseFuture};




pub struct WebsocketMessage {
    pub body: String,
}

pub struct WebsocketResponse {
    pub body: String,
}


impl Message for WebsocketMessage {
    type Result = Result<WebsocketResponse, ()>;
}

// impl Handler<WebsocketMessage> for WebsocketConnection {
//     type Result = ResponseActFuture<WebsocketConnection, Result<WebsocketResponse, ()>>;
//     fn handle(&mut self, msg: WebsocketMessage, ctx: &mut Self::Context) -> Self::Result {
//         // let f = self.send_message(msg.body);
//         // Box::pin(f)
//         Box::pin(Ok(()))
//     }
// }