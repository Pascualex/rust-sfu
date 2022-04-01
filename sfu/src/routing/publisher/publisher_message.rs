use crate::{routing::subscriber::SubscriberAddress, transport::Data};

pub enum PublisherMessage {
    Data(Data),
    Subscriber(SubscriberAddress),
    Keepalive,
    Stop,
}
