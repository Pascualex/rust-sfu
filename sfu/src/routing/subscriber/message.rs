use crate::transport::Media;

pub enum Message {
    Media(Media),
    Keepalive,
}
