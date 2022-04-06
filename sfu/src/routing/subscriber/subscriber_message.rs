use crate::endpoints::Data;

pub enum SubscriberMessage {
    Data(Data),
    Keepalive,
    Stop,
}
