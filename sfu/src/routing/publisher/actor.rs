use crate::{routing::subscriber::Subscriber, transport::Media};

pub struct Actor {
    subscribers: Vec<Subscriber>,
}

impl Actor {
    pub fn new(subscribers: Vec<Subscriber>) -> Self {
        Self { subscribers }
    }

    pub fn subscribe(&mut self, subscriber: Subscriber) {
        self.subscribers.push(subscriber);
    }

    pub async fn forward(&mut self, media: Media) {
        for subscriber in &self.subscribers {
            subscriber.send_media(media.clone()).await.ok(); // todo
        }
        self.subscribers.retain(|s| !s.is_closed());
    }

    pub fn keepalive(&mut self) {
        self.subscribers.retain(|s| !s.is_closed());
    }
}
