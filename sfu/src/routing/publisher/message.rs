use crate::{routing::subscriber::Subscriber, transport::Data};

pub enum Message {
    Subscription(Subscriber),
    Data(Data),
    Keepalive,
}
