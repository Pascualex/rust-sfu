use crate::transport::{MediaConsumer, MediaProducer};

pub enum Message {
    CreatePublisher(MediaProducer),
    CreateSubscriber(MediaConsumer),
    Keepalive,
}
