use crate::transport::Data;

pub enum SubscriberMessage {
    Data(Data),
    Keepalive,
    Stop,
}
