use crate::{
    routing::routing_error::RoutingError,
    transport::{Data, DataSender},
};

pub struct Subscriber {
    sender: DataSender,
}

impl Subscriber {
    pub fn new(sender: DataSender) -> Self {
        Self { sender }
    }

    pub async fn send(&mut self, data: Data) -> Result<(), RoutingError> {
        Ok(self.sender.send(data).await?)
    }

    pub async fn keepalive(&mut self) -> Result<(), RoutingError> {
        Ok(self.sender.keepalive().await?)
    }
}
