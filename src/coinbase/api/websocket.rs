use actix::{Message, Recipient};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize)]
pub struct WebsocketSubscription {
    #[serde(rename = "type")]
    pub message_type: String,
    pub product_ids: Vec<String>,
    pub channel: String,
    pub jwt: String,
}

#[derive(Serialize, Deserialize)]
pub struct WebsocketResponse {
    pub channel: String,
    pub client_id: String,
    pub timestamp: String,
    pub sequence_num: u32,
    pub events: Vec<Event>
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "type")]
    pub event_type: Option<String>,
    pub product_id: Option<String>,
    pub updates: Option<Vec<serde_json::Value>>
}

#[derive(Serialize, Deserialize)]
pub struct Update {

}
