use tokio::sync::mpsc::Sender;

use crate::transport::{MediaConsumer, MediaProducer};

use super::Message;

pub struct Address {
    sender: Sender<Message>,
}

impl Address {
    pub fn new(sender: Sender<Message>) -> Self {
        Self { sender }
    }

    pub async fn create_publisher(&self, producer: MediaProducer) {
        let message = Message::CreatePublisher(producer);
        self.sender.send(message).await.ok(); // todo
    }

    pub async fn create_subscriber(&self, consumer: MediaConsumer) {
        let message = Message::CreateSubscriber(consumer);
        self.sender.send(message).await.ok(); // todo
    }
}
