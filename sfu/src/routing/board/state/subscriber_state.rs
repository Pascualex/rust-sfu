use std::collections::HashSet;

use uuid::Uuid;

use crate::routing::subscriber::SubscriberAddress;

pub struct SubscriberState {
    pub address: SubscriberAddress,
    pub routers: HashSet<Uuid>,
}

impl SubscriberState {
    pub fn new(address: SubscriberAddress) -> Self {
        Self {
            address,
            routers: HashSet::new(),
        }
    }
}
