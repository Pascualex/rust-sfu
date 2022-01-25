use crate::transport::TransportError;

pub struct SendError;

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for SendError {
    fn from(_: tokio::sync::mpsc::error::SendError<T>) -> Self {
        SendError {}
    }
}

impl From<TransportError> for SendError {
    fn from(_: TransportError) -> Self {
        SendError {}
    }
}
