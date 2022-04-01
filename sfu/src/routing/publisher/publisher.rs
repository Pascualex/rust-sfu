use std::sync::Arc;

use crate::{
    routing::subscriber::{SubscriberAddress, SubscriberMessage},
    transport::Data,
};

pub struct Publisher {
    subscribers: Vec<SubscriberAddress>,
}

impl Publisher {
    pub fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }

    pub fn add_subscriber(&mut self, subscriber: SubscriberAddress) {
        self.subscribers.push(subscriber);
    }

    pub async fn send(&mut self, data: Data) {
        for subscriber in self.subscribers.iter() {
            let message = SubscriberMessage::Data(Arc::clone(&data));
            subscriber.send(message).await.ok(); // todo
        }
        self.subscribers.retain(|s| !s.is_closed());
    }

    pub fn keepalive(&mut self) {
        self.subscribers.retain(|s| !s.is_closed());
    }
}
