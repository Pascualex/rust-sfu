use tokio::sync::mpsc::Sender;

use crate::routing::{subscriber::Subscriber, SendError};

use super::Message;

pub struct Address {
    sender: Sender<Message>,
}

impl Address {
    pub fn new(sender: Sender<Message>) -> Self {
        Self { sender }
    }

    pub async fn subscribe(&self, subscriber: Subscriber) -> Result<(), SendError> {
        let message = Message::Subscription(subscriber);
        self.sender.send(message).await?;
        Ok(())
    }

    pub fn is_closed(&self) -> bool {
        self.sender.is_closed()
    }
}
