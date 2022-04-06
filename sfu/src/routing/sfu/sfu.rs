use std::collections::HashMap;

use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    endpoints::{PublisherEndpoint, SubscriberEndpoint},
    routing::{
        board::{board_loop, Board, BoardAddress, BoardMessage},
        publisher::{publisher_loop, Publisher, PublisherMessage},
        routing_error::RoutingError,
    },
};

use super::info::PublisherState;

pub struct Sfu {
    board: BoardAddress,
    publishers: HashMap<Uuid, PublisherState>,
}

impl Sfu {
    pub fn new() -> Self {
        let (address, mailbox) = mpsc::channel(100);
        let actor = Board::new();
        tokio::task::spawn(board_loop(actor, mailbox));

        Self {
            board: address,
            publishers: HashMap::new(),
        }
    }

    pub async fn create_publisher(&mut self, id: Uuid, endpoint: PublisherEndpoint) {
        let actor = Publisher::new(id, self.board.clone());
        let (address, mailbox) = mpsc::channel(100);
        tokio::task::spawn(publisher_loop(actor, mailbox, endpoint));

        let publisher = PublisherState::new(address);

        self.publishers.insert(id, publisher);
    }

    pub async fn create_subscriber(
        &mut self,
        id: Uuid,
        endpoint: SubscriberEndpoint,
    ) -> Result<(), RoutingError> {
        let message = BoardMessage::CreateSubscriber(id, endpoint);
        Ok(self.board.send(message).await?)
    }

    pub fn keepalive(&mut self) -> Result<(), RoutingError> {
        self.publishers.retain(|_, p| !p.address.is_closed());

        match self.board.is_closed() {
            true => Err(RoutingError::ChannelClosed),
            false => Ok(()),
        }
    }

    pub async fn stop(&mut self) {
        let message = BoardMessage::Stop;
        self.board.send(message).await.ok(); // todo

        for publisher in self.publishers.values() {
            let message = PublisherMessage::Stop;
            publisher.address.send(message).await.ok(); // todo
        }
    }
}
