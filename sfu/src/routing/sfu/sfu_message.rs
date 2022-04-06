use uuid::Uuid;

use crate::endpoints::{PublisherEndpoint, SubscriberEndpoint};

pub enum SfuMessage {
    CreatePublisher(Uuid, PublisherEndpoint),
    CreateSubscriber(Uuid, SubscriberEndpoint),
    Keepalive,
    Stop,
}
