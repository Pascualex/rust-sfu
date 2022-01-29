use crate::{
    routing::SendError,
    transport::{Consumer, Data},
};

pub struct Actor {
    consumer: Consumer,
}

impl Actor {
    pub fn new(consumer: Consumer) -> Self {
        Self { consumer }
    }

    pub async fn consume(&mut self, data: Data) -> Result<(), SendError> {
        self.consumer.send(data).await?;
        Ok(())
    }

    pub async fn keepalive(&mut self) -> Result<(), SendError> {
        self.consumer.keepalive().await?;
        Ok(())
    }
}
