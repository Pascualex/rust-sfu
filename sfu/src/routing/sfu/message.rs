use crate::transport::{Consumer, Producer};

pub enum Message {
    CreatePublisher(Producer),
    CreateSubscriber(Consumer),
    Keepalive,
}
