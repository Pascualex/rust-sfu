use std::{collections::HashMap, sync::Arc};

use uuid::Uuid;

use crate::{
    endpoints::Data,
    routing::subscriber::{SubscriberAddress, SubscriberMessage},
};

pub struct Router {
    pub id: Uuid,
    subscribers: HashMap<Uuid, SubscriberAddress>,
}

impl Router {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            subscribers: HashMap::new(),
        }
    }

    pub async fn route(&mut self, data: Data) {
        for subscriber in self.subscribers.values() {
            let message = SubscriberMessage::Data(self.id, Arc::clone(&data));
            subscriber.send(message).await.ok();
        }
        self.subscribers.retain(|_, s| !s.is_closed());
    }

    pub fn add_subscriber(&mut self, id: Uuid, subscriber: SubscriberAddress) {
        self.subscribers.insert(id, subscriber);
    }

    pub async fn keepalive(&mut self) {
        self.subscribers.retain(|_, s| !s.is_closed());
    }
}
