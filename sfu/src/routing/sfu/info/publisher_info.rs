use uuid::Uuid;

use crate::routing::publisher::PublisherAddress;

pub struct PublisherState {
    pub address: PublisherAddress,
    pub routers: Vec<Uuid>,
}

impl PublisherState {
    pub fn new(address: PublisherAddress) -> Self {
        Self {
            address,
            routers: Vec::new(),
        }
    }
}
