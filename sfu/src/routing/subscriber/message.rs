use crate::transport::Data;

pub enum Message {
    Data(Data),
    Keepalive,
}
