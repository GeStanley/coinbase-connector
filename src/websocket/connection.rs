use actix::io::SinkWrite;
use actix::prelude::*;
use actix_codec::Framed;
use awc::{BoxedSocket, Client, error::WsProtocolError, ws};
use bytestring::ByteString;
use futures::stream::{SplitSink, SplitStream};
use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use log::{error, info};
use openssl::ssl::SslConnector;
use ws::{Frame, Message, Codec};
use crate::websocket::market_data_handler::{MarketDataHandler, MessageReceivedRequest};

pub struct WebsocketClient {
    sink: SinkWrite<ws::Message, SplitSink<Framed<BoxedSocket, Codec>, ws::Message>>,
    subscriber: Addr<MarketDataHandler>,
}

impl WebsocketClient {
    pub fn start(subscriber: Addr<MarketDataHandler>, sink: SplitSink<Framed<BoxedSocket, Codec>, ws::Message>, stream: SplitStream<Framed<BoxedSocket, Codec>>) -> Addr<Self> {
        WebsocketClient::create(|ctx| {
            ctx.add_stream(stream);
            WebsocketClient {
                sink: SinkWrite::new(sink, ctx),
                subscriber,
            }
        })
    }
}

impl Actor for WebsocketClient {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        info!("Websocket client started");
    }
}

impl io::WriteHandler<WsProtocolError> for WebsocketClient {}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct WebsocketMessage {
    pub body: String,
}

impl Handler<WebsocketMessage> for WebsocketClient {
    type Result = ();

    fn handle(&mut self, msg: WebsocketMessage, _ctx: &mut Self::Context) {
        info!("Pushing Message {:?}", msg.body);
        match self
            .sink
            .write(Message::Text(ByteString::from(msg.body))) {
            Ok(_) => {
                info!("Message sent!");
            }
            Err(_) => {
                error!("Error sending message!");
            }
        }
    }
}

impl StreamHandler<Result<Frame, WsProtocolError>> for WebsocketClient {
    fn handle(&mut self, item: Result<Frame, WsProtocolError>, _ctx: &mut Self::Context) {
        match item.unwrap() {
            Frame::Text(text_bytes) => {
                self.subscriber.do_send(MessageReceivedRequest { msg: text_bytes, });
            }
            Frame::Binary(_) => {
                info!("Binary received!");
            }
            Frame::Continuation(_) => {
                info!("Continuation received!");
            }
            Frame::Ping(_) => {
                info!("Ping received!");
            }
            Frame::Pong(_) => {
                info!("Pong received!");
            }
            Frame::Close(_) => {
                info!("Close received!");
            }
        }
    }
}
