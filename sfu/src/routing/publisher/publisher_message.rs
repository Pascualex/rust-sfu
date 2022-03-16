use crate::{routing::subscriber::SubscriberAddress, transport::Data};

pub enum PublisherMessage {
    Subscription(SubscriberAddress),
    Data(Data),
    Keepalive,
}
