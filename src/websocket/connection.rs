use actix::io::SinkWrite;
use actix::prelude::*;
use actix_codec::Framed;
use awc::{error::WsProtocolError, ws, BoxedSocket, Client};
use bytestring::ByteString;
use futures::stream::{SplitSink, SplitStream};
use futures_util::stream::StreamExt;
use log::{error, info};
use openssl::ssl::SslConnector;

pub struct WebsocketClient {
    sink: SinkWrite<ws::Message, SplitSink<Framed<BoxedSocket, ws::Codec>, ws::Message>>,
}

impl WebsocketClient {
    pub fn start(sink: SplitSink<Framed<BoxedSocket, ws::Codec>, ws::Message>, stream: SplitStream<Framed<BoxedSocket, ws::Codec>>) -> Addr<Self> {
        WebsocketClient::create(|ctx| {
            ctx.add_stream(stream);
            WebsocketClient {
                sink: SinkWrite::new(sink, ctx),
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

impl actix::io::WriteHandler<WsProtocolError> for WebsocketClient {}

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
            .write(actix_http::ws::Message::Text(ByteString::from(msg.body))) {
            Ok(_) => {
                info!("Message sent!");
            }
            Err(_) => {
                error!("Error sending message!");
            }
        }
    }
}

impl StreamHandler<Result<ws::Frame, WsProtocolError>> for WebsocketClient {
    fn handle(&mut self, item: Result<ws::Frame, WsProtocolError>, _ctx: &mut Self::Context) {
        use ws::Frame;
        match item.unwrap() {
            Frame::Text(text_bytes) => {
                let text = std::str::from_utf8(text_bytes.as_ref()).unwrap();
                info!("Receiving Message: {}", text);
            }
            Frame::Binary(_) => {}
            Frame::Continuation(_) => {}
            Frame::Ping(_) => {
                info!("Ping received!");
            }
            Frame::Pong(_) => {
                //self.hb = Instant::now();
            }
            Frame::Close(_) => {}
        }
    }
}
