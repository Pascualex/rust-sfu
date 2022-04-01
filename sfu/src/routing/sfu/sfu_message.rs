use crate::transport::{DataReceiver, DataSender};

pub enum SfuMessage {
    CreatePublisher(DataReceiver),
    CreateSubscriber(DataSender),
    Keepalive,
    Stop,
}
