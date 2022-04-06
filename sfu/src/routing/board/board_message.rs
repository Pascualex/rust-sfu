use uuid::Uuid;

use crate::{endpoints::SubscriberEndpoint, routing::router::RouterChannel};

pub enum BoardMessage {
    CreateSubscriber(Uuid, SubscriberEndpoint),
    CreateRouter(Uuid, Uuid, RouterChannel),
    Keepalive,
    Stop,
}
