use tokio::{sync::mpsc, task};

use crate::{
    routing::{
        publisher::{publisher_loop, Publisher, PublisherAddress},
        subscriber::{subscriber_loop, Subscriber, SubscriberAddress, SubscriberMessage},
    },
    transport::{DataReceiver, DataSender},
};

pub struct Sfu {
    publisher_addrs: Vec<PublisherAddress>,
    subscriber_addrs: Vec<SubscriberAddress>,
}

impl Sfu {
    pub fn new() -> Self {
        Self {
            publisher_addrs: Vec::new(),
            subscriber_addrs: Vec::new(),
        }
    }

    pub async fn create_publisher(&mut self, data_rcvr: DataReceiver) {
        let (publisher_addr, publisher_mailbox) = mpsc::channel(100);
        let publisher = Publisher::new();
        task::spawn(publisher_loop(publisher, publisher_mailbox, data_rcvr));

        for subscriber_addr in self.subscriber_addrs.iter() {
            let message = SubscriberMessage::Subscription(publisher_addr.clone());
            subscriber_addr.send(message).await.ok(); // todo
        }
        self.publisher_addrs.push(publisher_addr);
    }

    pub async fn create_subscriber(&mut self, data_sndr: DataSender) {
        let (subscriber_addr, subscriber_mailbox) = mpsc::channel(100);
        let subscriber = Subscriber::new(subscriber_addr.clone(), data_sndr);
        task::spawn(subscriber_loop(subscriber, subscriber_mailbox));

        for publisher_addr in self.publisher_addrs.iter() {
            let message = SubscriberMessage::Subscription(publisher_addr.clone());
            subscriber_addr.send(message).await.ok(); // todo
        }

        self.subscriber_addrs.push(subscriber_addr);
    }

    pub fn keepalive(&mut self) {
        self.publisher_addrs.retain(|p| !p.is_closed());
        self.subscriber_addrs.retain(|s| !s.is_closed());
    }
}
