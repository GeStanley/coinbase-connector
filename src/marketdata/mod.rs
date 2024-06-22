pub mod order_book;


#[cfg(test)]
mod tests {
    use std::fs;
    use std::str::FromStr;
    use bigdecimal::BigDecimal;

    use chrono::DateTime;
    use chrono::prelude::*;

    use crate::coinbase::api::websocket::{Event, WebsocketResponse};
    use crate::marketdata::order_book::{Book, Order};

    #[test]
    fn test_market_data_insert() {
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("RUST_BACKTRACE", "full");

        let mut order_book: Book = Book::new("some_product".parse().unwrap());

        let snapshot = "snapshot.json";
        let snapshot_contents = fs::read_to_string(snapshot);
        let websocket_snapshot: WebsocketResponse = serde_json::from_str(&*snapshot_contents.unwrap()).unwrap();

        insert_events_into_order_book(websocket_snapshot.events, &mut order_book);

        assert_eq!(order_book.bids.iter().size_hint(), (2, Some(2)));
        assert_eq!(order_book.offers.iter().size_hint(), (2, Some(2)));

        assert_eq!(order_book.bids.get(&BigDecimal::from_str("0.04911").unwrap()).unwrap().quantity, BigDecimal::from_str("0.872").unwrap());
        assert_eq!(order_book.offers.get(&BigDecimal::from_str("0.04912").unwrap()).unwrap().quantity, BigDecimal::from_str("12.36374816").unwrap());

        let update = "update.json";
        let snapshot_contents = fs::read_to_string(update);
        let websocket_update: WebsocketResponse = serde_json::from_str(&*snapshot_contents.unwrap()).unwrap();

        insert_events_into_order_book(websocket_update.events, &mut order_book);

        assert_eq!(order_book.bids.iter().size_hint(), (2, Some(2)));
        assert_eq!(order_book.offers.iter().size_hint(), (4, Some(4)));

        assert_eq!(order_book.bids.get(&BigDecimal::from_str("0.04911").unwrap()).unwrap().quantity, BigDecimal::from_str("0.872").unwrap());
        assert_eq!(order_book.offers.get(&BigDecimal::from_str("0.04912").unwrap()).unwrap().quantity, BigDecimal::from_str("12.36474816").unwrap());
        assert_eq!(order_book.offers.get(&BigDecimal::from_str("0.04916").unwrap()).unwrap().quantity, BigDecimal::from_str("19.33301376").unwrap());
    }

    fn insert_events_into_order_book(events: Vec<Event>, order_book: &mut Book) {
        for event in events.iter() {
            for update in event.updates.iter() {
                for element in update.iter() {
                    let qty: &str = &element["new_quantity"].as_str().unwrap();
                    let price: &str = &element["price_level"].as_str().unwrap();
                    let timestamp: DateTime<Utc> = (&element["event_time"].as_str().unwrap()).parse::<DateTime<Utc>>().unwrap();
                    let side: &str = &element["side"].as_str().unwrap();
                    order_book.insert_update(side, price, qty, timestamp);
                }
            }
        }
    }
}