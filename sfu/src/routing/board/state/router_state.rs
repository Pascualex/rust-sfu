use std::collections::HashSet;

use uuid::Uuid;

use crate::routing::router::RouterAddress;

pub struct RouterState {
    pub address: RouterAddress,
    pub publisher: Uuid,
    pub subscribers: HashSet<Uuid>,
}

impl RouterState {
    pub fn new(address: RouterAddress, publisher: Uuid, subscribers: HashSet<Uuid>) -> Self {
        Self {
            address,
            publisher,
            subscribers,
        }
    }
}
