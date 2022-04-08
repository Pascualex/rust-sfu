use uuid::Uuid;

use crate::endpoints::Data;

pub enum SubscriberMessage {
    Data(Uuid, Data),
    Keepalive,
    Stop,
}
