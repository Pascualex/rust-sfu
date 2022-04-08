use uuid::Uuid;

use crate::endpoints::Data;

pub enum PublisherMessage {
    Data(Uuid, Data),
    Keepalive,
    Stop,
}
