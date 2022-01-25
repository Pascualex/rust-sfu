use crate::{
    routing::{spawn_publisher, spawn_subscriber, Publisher, Subscriber},
    transport::{MediaConsumer, MediaProducer},
};

pub struct Actor {
    publishers: Vec<Publisher>,
    subscribers: Vec<Subscriber>,
}

impl Actor {
    pub fn new() -> Self {
        Self {
            publishers: Vec::new(),
            subscribers: Vec::new(),
        }
    }

    pub async fn create_publisher(&mut self, producer: MediaProducer) {
        let publisher = spawn_publisher(producer, self.subscribers.clone());
        self.publishers.push(publisher);
    }

    pub async fn create_subscriber(&mut self, consumer: MediaConsumer) {
        let subscriber = spawn_subscriber(consumer);
        for publisher in self.publishers.iter() {
            publisher.subscribe(subscriber.clone()).await.ok(); // todo
        }
        self.publishers.retain(|p| !p.is_closed());
        self.subscribers.push(subscriber);
    }

    pub fn keepalive(&mut self) {
        self.publishers.retain(|p| !p.is_closed());
        self.subscribers.retain(|s| !s.is_closed());
    }
}
