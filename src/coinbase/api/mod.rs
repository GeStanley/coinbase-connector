pub mod http;
pub mod websocket;

#[cfg(test)]
mod tests {
    use std::fs;
    use serde_json::Value;
    use crate::coinbase::api::websocket::{CoinbaseWebsocketMessage, Event, MarketData};

    #[test]
    fn test_coinbase_snapshot_message_parsing() {
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("RUST_BACKTRACE", "full");

        let snapshot = "snapshot.json";
        let snapshot_contents = fs::read_to_string(snapshot);
        let websocket_snapshot: CoinbaseWebsocketMessage = serde_json::from_str(&*snapshot_contents.unwrap()).unwrap();


        assert_eq!(websocket_snapshot.events.iter().size_hint(), (1, Some(1)));

        let mut iter = websocket_snapshot.events.iter();
        let event = iter.next().expect("this is a test!");

        match event {
            Event::MarketData(data) => {
                match data {
                    MarketData::update { .. } => {
                        assert_eq!(true, false);
                    }
                    MarketData::snapshot { product_id, updates } => {
                        assert_eq!(product_id, &Some("ETH-BTC".to_string()));
                    }
                }
            }
            Event::Subscription(_) => {
                assert_eq!(true, false);
            }
        }
    }

    #[test]
    fn test_coinbase_update_message_parsing() {
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("RUST_BACKTRACE", "full");

        let update = "update.json";
        let snapshot_contents = fs::read_to_string(update);
        let websocket_update: CoinbaseWebsocketMessage = serde_json::from_str(&*snapshot_contents.unwrap()).unwrap();

        assert_eq!(websocket_update.events.iter().size_hint(), (1, Some(1)));

        let mut iter = websocket_update.events.iter();
        let event = iter.next().expect("this is a test!");

        match event {
            Event::MarketData(data) => {
                match data {
                    MarketData::update { product_id, updates } => {
                        assert_eq!(product_id, &Some("ETH-BTC".to_string()));
                    }
                    MarketData::snapshot { .. } => {
                        assert_eq!(true, false);
                    }
                }
            }
            Event::Subscription(_) => {
                assert_eq!(true, false);
            }
        }
    }

    #[test]
    fn test_coinbase_subscription_message_parsing() {
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("RUST_BACKTRACE", "full");

        let update = "subscriptions.json";
        let snapshot_contents = fs::read_to_string(update);
        let websocket_subscription: CoinbaseWebsocketMessage = serde_json::from_str(&*snapshot_contents.unwrap()).unwrap();

        assert_eq!(websocket_subscription.events.iter().size_hint(), (1, Some(1)));

        let mut iter = websocket_subscription.events.iter();
        let event = iter.next().expect("this is a test!");

        match event {
            Event::MarketData(_) => {
                assert_eq!(true, false);
            }
            Event::Subscription(sub) => {
                // let data= &sub.subscriptions.expect("this is a test!")["level2"];
            }
        }
    }
}