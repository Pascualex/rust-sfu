use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    routing::{
        board::{BoardAddress, BoardMessage},
        router::{RouterAddress, RouterMessage},
    },
    transport::Data,
};

pub struct Publisher {
    pub id: Uuid,
    routers: Vec<RouterAddress>,
    board: BoardAddress,
}

impl Publisher {
    pub fn new(id: Uuid, board: BoardAddress) -> Self {
        Self {
            id,
            routers: Vec::new(),
            board,
        }
    }

    pub async fn route(&mut self, data: Data) {
        if self.routers.is_empty() {
            let id = Uuid::new_v4();
            let (address, mailbox) = mpsc::channel(100);

            self.routers.push(address.clone());

            let channel = (address, mailbox);
            let message = BoardMessage::CreateRouter(id, self.id, channel);
            self.board.send(message).await.ok(); // todo
        }

        let message = RouterMessage::Data(data);
        self.routers[0].send(message).await.ok(); // todo
    }

    pub fn keepalive(&mut self) {
        // todo
    }

    pub async fn stop(&mut self) {
        for router in self.routers.iter() {
            let message = RouterMessage::Stop;
            router.send(message).await.ok(); // todo
        }
    }
}
