use crate::{routing::publisher::PublisherAddress, transport::Data};

pub enum SubscriberMessage {
    Subscription(PublisherAddress),
    Data(Data),
    Keepalive,
}
