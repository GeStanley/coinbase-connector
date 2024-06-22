use either::Either;
use actix::Message;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CoinbaseWebsocketSubscription {
    #[serde(rename = "type")]
    pub message_type: String,
    pub product_ids: Vec<String>,
    pub channel: String,
    pub jwt: String,
}

#[derive(Serialize, Deserialize)]
pub struct CoinbaseWebsocketMessage {
    pub channel: String,
    pub client_id: String,
    pub timestamp: String,
    pub sequence_num: u32,
    pub events: Vec<Event>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Event {
    MarketData ( MarketData ),
    Subscription ( Subscription ),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MarketData {
    update { product_id: Option<String>, updates: Option<Vec<Update>> },
    snapshot { product_id: Option<String>, updates: Option<Vec<Update>> },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "side")]
pub enum Update {
    bid { event_time: String, price_level: String, new_quantity: String },
    offer { event_time: String, price_level: String, new_quantity: String },
}

#[derive(Serialize, Deserialize)]
pub struct Subscription {
    pub subscriptions: Option<serde_json::Value>,
}
