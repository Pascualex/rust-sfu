use crate::{
    routing::{
        publisher::{publisher_loop, Publisher, PublisherAddress, PublisherMessage},
        subscriber::{subscriber_loop, Subscriber, SubscriberAddress, SubscriberMessage},
        MAX_SUBSCRIPTIONS,
    },
    transport::{DataReceiver, DataSender},
};

pub struct Sfu {
    publishers: Vec<PublisherAddress>,
    subscribers: Vec<(SubscriberAddress, Vec<PublisherAddress>)>,
}

impl Sfu {
    pub fn new() -> Self {
        Self {
            publishers: Vec::new(),
            subscribers: Vec::new(),
        }
    }

    pub async fn create_publisher(&mut self, receiver: DataReceiver) {
        let (address, mailbox) = tokio::sync::mpsc::channel(100);
        let publisher = Publisher::new();
        tokio::task::spawn(publisher_loop(publisher, mailbox, receiver));

        let publisher = address;

        for (subscriber, subscriptions) in self.subscribers.iter_mut() {
            Self::subscribe(subscriber, &publisher, subscriptions).await;
        }

        self.publishers.push(publisher);
    }

    pub async fn create_subscriber(&mut self, sender: DataSender) {
        let (address, mailbox) = tokio::sync::mpsc::channel(100);
        let subscriber = Subscriber::new(sender);
        tokio::task::spawn(subscriber_loop(subscriber, mailbox));

        let subscriber = address;
        let mut subscriptions = Vec::new();

        for publisher in self.publishers.iter() {
            Self::subscribe(&subscriber, publisher, &mut subscriptions).await;
        }

        self.subscribers.push((subscriber, subscriptions));
    }

    pub async fn subscribe(
        subscriber: &SubscriberAddress,
        publisher: &PublisherAddress,
        subscriptions: &mut Vec<PublisherAddress>,
    ) {
        if subscriptions.len() < MAX_SUBSCRIPTIONS {
            let message = PublisherMessage::Subscriber(subscriber.clone());
            publisher.send(message).await.ok(); // todo

            subscriptions.push(publisher.clone());
        }
    }

    pub fn keepalive(&mut self) {
        self.publishers.retain(|p| !p.is_closed());
        self.subscribers.retain(|(s, _)| !s.is_closed());
    }

    pub async fn stop(&mut self) {
        for publisher in self.publishers.iter() {
            let message = PublisherMessage::Stop;
            publisher.send(message).await.ok(); // todo
        }

        for (subscriber, _) in self.subscribers.iter() {
            let message = SubscriberMessage::Stop;
            subscriber.send(message).await.ok(); // todo
        }
    }
}
