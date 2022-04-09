use std::collections::HashMap;

use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    endpoints::{PublisherEndpoint, SubscriberEndpoint},
    routing::{
        publisher::{publisher_loop, Publisher, PublisherMessage},
        routing_error::RoutingError,
        subscription_manager::{
            subscription_manager_loop, SubscriptionManager, SubscriptionManagerAddress,
            SubscriptionManagerMessage,
        },
    },
};

use super::info::PublisherState;

pub struct Sfu {
    subscription_manager: SubscriptionManagerAddress,
    publishers: HashMap<Uuid, PublisherState>,
}

impl Sfu {
    pub fn new() -> Self {
        let (address, mailbox) = mpsc::channel(100);
        let actor = SubscriptionManager::new();
        tokio::task::spawn(subscription_manager_loop(actor, mailbox));

        Self {
            subscription_manager: address,
            publishers: HashMap::new(),
        }
    }

    pub async fn create_publisher(&mut self, id: Uuid, endpoint: PublisherEndpoint) {
        let actor = Publisher::new(id, self.subscription_manager.clone());
        let (address, mailbox) = mpsc::channel(100);
        tokio::task::spawn(publisher_loop(actor, mailbox, endpoint));

        let publisher = PublisherState::new(address);

        self.publishers.insert(id, publisher);
    }

    pub async fn create_subscriber(
        &mut self,
        id: Uuid,
        endpoint: SubscriberEndpoint,
    ) -> Result<(), RoutingError> {
        let message = SubscriptionManagerMessage::CreateSubscriber(id, endpoint);
        Ok(self.subscription_manager.send(message).await?)
    }

    pub fn keepalive(&mut self) -> Result<(), RoutingError> {
        self.publishers.retain(|_, p| !p.address.is_closed());

        match self.subscription_manager.is_closed() {
            true => Err(RoutingError::ChannelClosed),
            false => Ok(()),
        }
    }

    pub async fn stop(&mut self) {
        let message = SubscriptionManagerMessage::Stop;
        self.subscription_manager.send(message).await.ok(); // todo

        for publisher in self.publishers.values() {
            let message = PublisherMessage::Stop;
            publisher.address.send(message).await.ok(); // todo
        }
    }
}
