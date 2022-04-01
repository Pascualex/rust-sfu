use tokio::sync::mpsc::error::SendError;

use crate::transport::TransportError;

pub enum RoutingError {
    ChannelClosed,
}

impl From<TransportError> for RoutingError {
    fn from(_: TransportError) -> Self {
        Self::ChannelClosed
    }
}

impl<T> From<SendError<T>> for RoutingError {
    fn from(_: SendError<T>) -> Self {
        Self::ChannelClosed
    }
}
