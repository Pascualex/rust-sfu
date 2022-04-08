use std::collections::{hash_map::Entry, HashMap};

use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    endpoints::Data,
    routing::{
        board::{BoardAddress, BoardMessage},
        router::{RouterAddress, RouterMessage},
    },
};

pub struct Publisher {
    pub id: Uuid,
    routers: HashMap<Uuid, RouterAddress>,
    board: BoardAddress,
}

impl Publisher {
    pub fn new(id: Uuid, board: BoardAddress) -> Self {
        Self {
            id,
            routers: HashMap::new(),
            board,
        }
    }

    pub async fn route(&mut self, track_id: Uuid, data: Data) {
        let router = match self.routers.entry(track_id) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                let (address, mailbox) = mpsc::channel(100);
                let channel = (address.clone(), mailbox);
                let message = BoardMessage::CreateRouter(track_id, self.id, channel);
                self.board.send(message).await.ok(); // todo
                v.insert(address)
            }
        };

        let message = RouterMessage::Data(data);
        router.send(message).await.ok(); // todo
    }

    pub fn keepalive(&mut self) {
        // todo
    }

    pub async fn stop(&mut self) {
        for router in self.routers.values() {
            let message = RouterMessage::Stop;
            router.send(message).await.ok(); // todo
        }
    }
}
