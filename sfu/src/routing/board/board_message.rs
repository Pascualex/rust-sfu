use uuid::Uuid;

use crate::{transport::DataSender, routing::router::RouterChannel};

pub enum BoardMessage {
    CreateSubscriber(Uuid, DataSender),
    CreateRouter(Uuid, Uuid, RouterChannel),
    Keepalive,
    Stop,
}
