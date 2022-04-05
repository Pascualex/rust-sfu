use uuid::Uuid;

use crate::transport::{DataReceiver, DataSender};

pub enum SfuMessage {
    CreatePublisher(Uuid, DataReceiver),
    CreateSubscriber(Uuid, DataSender),
    Keepalive,
    Stop,
}
