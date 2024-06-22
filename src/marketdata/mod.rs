pub mod order_book;


#[cfg(test)]
mod tests {
    use std::fs;
    use std::str::FromStr;

    use bigdecimal::BigDecimal;
    use chrono::DateTime;
    use chrono::prelude::*;

    use crate::coinbase::api::websocket::{CoinbaseWebsocketMessage, Event, MarketData, Update};
    use crate::marketdata::order_book::Book;

    #[test]
    fn test_market_data_insert() {
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("RUST_BACKTRACE", "full");

        let mut order_book: Book = Book::new("some_product".parse().unwrap());

        let snapshot = "snapshot.json";
        let snapshot_contents = fs::read_to_string(snapshot);
        let websocket_snapshot: CoinbaseWebsocketMessage = serde_json::from_str(&*snapshot_contents.unwrap()).unwrap();

        insert_events_into_order_book(websocket_snapshot.events, &mut order_book);

        assert_eq!(order_book.bids.iter().size_hint(), (2, Some(2)));
        assert_eq!(order_book.offers.iter().size_hint(), (2, Some(2)));

        assert_eq!(order_book.bids.get(&BigDecimal::from_str("0.04911").unwrap()).unwrap().quantity, BigDecimal::from_str("0.872").unwrap());
        assert_eq!(order_book.offers.get(&BigDecimal::from_str("0.04912").unwrap()).unwrap().quantity, BigDecimal::from_str("12.36374816").unwrap());

        let update = "update.json";
        let snapshot_contents = fs::read_to_string(update);
        let websocket_update: CoinbaseWebsocketMessage = serde_json::from_str(&*snapshot_contents.unwrap()).unwrap();

        insert_events_into_order_book(websocket_update.events, &mut order_book);

        assert_eq!(order_book.bids.iter().size_hint(), (2, Some(2)));
        assert_eq!(order_book.offers.iter().size_hint(), (4, Some(4)));

        assert_eq!(order_book.bids.get(&BigDecimal::from_str("0.04911").unwrap()).unwrap().quantity, BigDecimal::from_str("0.872").unwrap());
        assert_eq!(order_book.offers.get(&BigDecimal::from_str("0.04912").unwrap()).unwrap().quantity, BigDecimal::from_str("12.36474816").unwrap());
        assert_eq!(order_book.offers.get(&BigDecimal::from_str("0.04916").unwrap()).unwrap().quantity, BigDecimal::from_str("19.33301376").unwrap());
    }

    fn insert_events_into_order_book(events: Vec<Event>, order_book: &mut Book) {
        for event in events.iter() {
            match event {
                Event::MarketData(e) => {
                    match e {
                        MarketData::update { product_id, updates } => {
                            for update in updates.iter() {
                                for element in update.iter() {
                                    match element {
                                        Update::bid { event_time, price_level, new_quantity } => {
                                            order_book.insert_update("bid", price_level, new_quantity, event_time.parse::<DateTime<Utc>>().unwrap());
                                        }
                                        Update::offer { event_time, price_level, new_quantity } => {
                                            order_book.insert_update("offer", price_level, new_quantity, event_time.parse::<DateTime<Utc>>().unwrap());
                                        }
                                    }
                                }
                            }
                        }
                        MarketData::snapshot { product_id, updates } => {}
                    }
                }
                Event::Subscription(e) => {}
            }
        }
    }
}