use uuid::Uuid;

use crate::{
    endpoints::{Data, SubscriberEndpoint},
    routing::routing_error::RoutingError,
};

pub struct Subscriber {
    pub id: Uuid,
    endpoint: SubscriberEndpoint,
}

impl Subscriber {
    pub fn new(id: Uuid, endpoint: SubscriberEndpoint) -> Self {
        Self { id, endpoint }
    }

    pub async fn send(&mut self, data: Data) -> Result<(), RoutingError> {
        Ok(self.endpoint.send(data).await?)
    }

    pub async fn keepalive(&mut self) -> Result<(), RoutingError> {
        Ok(self.endpoint.keepalive().await?)
    }
}
