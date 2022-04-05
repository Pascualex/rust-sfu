use uuid::Uuid;

use crate::{
    routing::routing_error::RoutingError,
    transport::{Data, DataSender},
};

pub struct Subscriber {
    pub id: Uuid,
    sender: DataSender,
}

impl Subscriber {
    pub fn new(id: Uuid, sender: DataSender) -> Self {
        Self { id, sender }
    }

    pub async fn send(&mut self, data: Data) -> Result<(), RoutingError> {
        Ok(self.sender.send(data).await?)
    }

    pub async fn keepalive(&mut self) -> Result<(), RoutingError> {
        Ok(self.sender.keepalive().await?)
    }
}
