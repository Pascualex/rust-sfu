use uuid::Uuid;

use crate::{endpoints::Data, routing::subscriber::SubscriberAddress};

pub enum RouterMessage {
    Data(Data),
    Subscriber(Uuid, SubscriberAddress),
    Keepalive,
    Stop,
}
