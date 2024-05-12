use actix::{Actor, Context, Handler, Message, Recipient};
use uuid::Uuid;

use crate::marketdata::order_book::Book;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub product_id: String,
    pub self_id: Uuid,
}


#[derive(Message)]
#[rtype(result = "()")]
pub struct MarketDataSnapshot {
    pub id: Uuid,
    pub msg: String,
    pub room_id: Uuid
}


#[derive(Message)]
#[rtype(result = "()")]
pub struct MarketDataUpdate {
    pub id: Uuid,
    pub msg: String,
    pub room_id: Uuid
}

pub struct CoinbaseMarketData {
    order_book: Book,
}

impl Default for CoinbaseMarketData {
    fn default() -> CoinbaseMarketData {
        CoinbaseMarketData {
            order_book: Book::new("ETC-BTC".parse().unwrap()),
        }
    }
}

impl Actor for CoinbaseMarketData {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        println!("order book started!");
    }
}

impl Handler<Connect> for CoinbaseMarketData {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Context<Self>) -> Self::Result {
        println!("connect!");
    }
}

impl Handler<MarketDataSnapshot> for CoinbaseMarketData {
    type Result = ();

    fn handle(&mut self, msg: MarketDataSnapshot, _ctx: &mut Context<Self>) -> Self::Result {
        println!("snapshot!");
    }
}

impl Handler<MarketDataUpdate> for CoinbaseMarketData {
    type Result = ();

    fn handle(&mut self, msg: MarketDataUpdate, _ctx: &mut Context<Self>) -> Self::Result {
        println!("update!")
    }
}