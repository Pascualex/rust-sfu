use std::collections::{HashMap, HashSet};

use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    endpoints::SubscriberEndpoint,
    routing::{
        router::{router_loop, Router, RouterChannel, RouterMessage},
        subscriber::{subscriber_loop, Subscriber, SubscriberMessage},
        MAX_SUBSCRIPTIONS,
    },
};

use super::state::{RouterState, SubscriberState};

pub struct SubscriptionManager {
    routers: HashMap<Uuid, RouterState>,
    subscribers: HashMap<Uuid, SubscriberState>,
}

impl SubscriptionManager {
    pub fn new() -> Self {
        Self {
            routers: HashMap::new(),
            subscribers: HashMap::new(),
        }
    }

    pub async fn create_router(&mut self, id: Uuid, publisher_id: Uuid, channel: RouterChannel) {
        let mut actor = Router::new(publisher_id);
        let mut router_subscribers = HashSet::new();

        for (subscriber_id, subscriber) in self.subscribers.iter_mut() {
            if *subscriber_id != publisher_id && subscriber.routers.len() < MAX_SUBSCRIPTIONS {
                let subscriber_address = subscriber.address.clone();
                actor.add_subscriber(*subscriber_id, subscriber_address);

                router_subscribers.insert(*subscriber_id);
                subscriber.routers.insert(id);
            }
        }

        let (address, mailbox) = channel;
        tokio::task::spawn(router_loop(actor, mailbox));

        let router = RouterState::new(address, publisher_id, router_subscribers);
        self.routers.insert(id, router);
    }

    pub async fn create_subscriber(&mut self, id: Uuid, endpoint: SubscriberEndpoint) {
        let actor = Subscriber::new(id, endpoint);
        let (address, mailbox) = mpsc::channel(100);
        tokio::task::spawn(subscriber_loop(actor, mailbox));

        let mut subscriber = SubscriberState::new(address);

        for (router_id, router) in self.routers.iter_mut() {
            if router.publisher != id && subscriber.routers.len() < MAX_SUBSCRIPTIONS {
                let subscriber_address = subscriber.address.clone();
                let message = RouterMessage::Subscriber(id, subscriber_address);
                router.address.send(message).await.ok(); // todo

                router.subscribers.insert(id);
                subscriber.routers.insert(*router_id);
            }
        }

        self.subscribers.insert(id, subscriber);
    }

    pub fn keepalive(&mut self) {
        self.routers.retain(|_, r| !r.address.is_closed());
        self.subscribers.retain(|_, s| !s.address.is_closed());
    }

    pub async fn stop(&mut self) {
        for router in self.routers.values() {
            let message = RouterMessage::Stop;
            router.address.send(message).await.ok(); // todo
        }

        for subscriber in self.subscribers.values() {
            let message = SubscriberMessage::Stop;
            subscriber.address.send(message).await.ok(); // todo
        }
    }
}
