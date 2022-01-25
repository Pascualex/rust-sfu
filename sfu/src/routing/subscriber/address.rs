use tokio::sync::mpsc::Sender;

use crate::{routing::SendError, transport::Media};

use super::message::Message;

#[derive(Clone)]
pub struct Address {
    sender: Sender<Message>,
}

impl Address {
    pub fn new(sender: Sender<Message>) -> Self {
        Self { sender }
    }

    pub async fn send_media(&self, media: Media) -> Result<(), SendError> {
        self.sender.send(Message::Media(media)).await?;
        Ok(())
    }

    pub fn is_closed(&self) -> bool {
        self.sender.is_closed()
    }
}
