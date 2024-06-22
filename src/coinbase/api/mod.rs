pub mod http;
pub mod websocket;

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::coinbase::api::websocket::{CoinbaseWebsocketMessage, Event};

    #[test]
    fn test_coinbase_message_parsing() {
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("RUST_BACKTRACE", "full");

        let snapshot = "snapshot.json";
        let snapshot_contents = fs::read_to_string(snapshot);
        let websocket_snapshot: CoinbaseWebsocketMessage = serde_json::from_str(&*snapshot_contents.unwrap()).unwrap();


        assert_eq!(websocket_snapshot.events.iter().size_hint(), (1, Some(1)));

        let mut iter = websocket_snapshot.events.iter();
        let event = iter.next();

        let update = "update.json";
        let snapshot_contents = fs::read_to_string(update);
        let websocket_update: CoinbaseWebsocketMessage = serde_json::from_str(&*snapshot_contents.unwrap()).unwrap();

        assert_eq!(websocket_update.events.iter().size_hint(), (1, Some(1)));

        let update = "subscriptions.json";
        let snapshot_contents = fs::read_to_string(update);
        let websocket_subscription: CoinbaseWebsocketMessage = serde_json::from_str(&*snapshot_contents.unwrap()).unwrap();

        assert_eq!(websocket_update.events.iter().size_hint(), (1, Some(1)));
    }
}