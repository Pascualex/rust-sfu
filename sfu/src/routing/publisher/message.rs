use crate::{routing::subscriber::Subscriber, transport::Media};

pub enum Message {
    Subscription(Subscriber),
    Media(Media),
    Keepalive,
}
