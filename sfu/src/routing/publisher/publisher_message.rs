use crate::transport::Data;

pub enum PublisherMessage {
    Data(Data),
    Keepalive,
    Stop,
}
