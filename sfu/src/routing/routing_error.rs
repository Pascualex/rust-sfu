use tokio::sync::mpsc::error::SendError;

use crate::endpoints::EndpointError;

pub enum RoutingError {
    ChannelClosed,
}

impl From<EndpointError> for RoutingError {
    fn from(_: EndpointError) -> Self {
        Self::ChannelClosed
    }
}

impl<T> From<SendError<T>> for RoutingError {
    fn from(_: SendError<T>) -> Self {
        Self::ChannelClosed
    }
}
