use tokio::sync::mpsc::Sender;

use crate::{routing::SendError, transport::Data};

use super::message::Message;

#[derive(Clone)]
pub struct Address {
    sender: Sender<Message>,
}

impl Address {
    pub fn new(sender: Sender<Message>) -> Self {
        Self { sender }
    }

    pub async fn send_data(&self, data: Data) -> Result<(), SendError> {
        self.sender.send(Message::Data(data)).await?;
        Ok(())
    }

    pub fn is_closed(&self) -> bool {
        self.sender.is_closed()
    }
}
