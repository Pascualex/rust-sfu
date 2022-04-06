use crate::endpoints::Data;

pub enum PublisherMessage {
    Data(Data),
    Keepalive,
    Stop,
}
