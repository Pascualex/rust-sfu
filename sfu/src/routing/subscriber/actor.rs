use crate::{
    routing::SendError,
    transport::{Media, MediaConsumer},
};

pub struct Actor {
    consumer: MediaConsumer,
}

impl Actor {
    pub fn new(consumer: MediaConsumer) -> Self {
        Self { consumer }
    }

    pub async fn consume(&mut self, media: Media) -> Result<(), SendError> {
        self.consumer.send(media).await?;
        Ok(())
    }

    pub async fn keepalive(&mut self) -> Result<(), SendError> {
        self.consumer.keepalive().await?;
        Ok(())
    }
}
