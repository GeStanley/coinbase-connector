use std::time::Instant;
use actix::Addr;
use uuid::Uuid;
use crate::coinbase::lobby::Lobby;


pub struct Conn {
    addr: Addr<Lobby>,
    hb: Instant,
    id: Uuid,
}

impl Conn {
    pub fn new(lobby: Addr<Lobby>) -> Conn {
        Conn {
            id: Uuid::new_v4(),
            hb: Instant::now(),
            addr: lobby,
        }
    }
}