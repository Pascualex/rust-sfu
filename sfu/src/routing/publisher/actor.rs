use crate::{routing::subscriber::Subscriber, transport::Data};

pub struct Actor {
    subscribers: Vec<Subscriber>,
}

impl Actor {
    pub fn new(subscribers: Vec<Subscriber>) -> Self {
        Self {
            subscribers: subscribers.into_iter().take(49).collect(),
        }
    }

    pub fn subscribe(&mut self, subscriber: Subscriber) {
        if self.subscribers.len() < 49 {
            self.subscribers.push(subscriber);
        }
    }

    pub async fn forward(&mut self, data: Data) {
        for subscriber in &self.subscribers {
            subscriber.send_data(data.clone()).await.ok(); // todo
        }
        self.subscribers.retain(|s| !s.is_closed());
    }

    pub fn keepalive(&mut self) {
        self.subscribers.retain(|s| !s.is_closed());
    }
}
