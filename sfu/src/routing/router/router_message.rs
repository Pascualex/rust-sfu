use uuid::Uuid;

use crate::{routing::subscriber::SubscriberAddress, transport::Data};

pub enum RouterMessage {
    Data(Data),
    Subscriber(Uuid, SubscriberAddress),
    Keepalive,
    Stop,
}
