use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::{stream::SplitStream, Stream, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use super::Media;

pub struct MediaProducer {
    split_stream: SplitStream<WebSocketStream<TcpStream>>,
}

impl MediaProducer {
    pub fn new(split_stream: SplitStream<WebSocketStream<TcpStream>>) -> Self {
        Self { split_stream }
    }
}

impl Stream for MediaProducer {
    type Item = Media;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.split_stream.poll_next_unpin(cx) {
            Poll::Ready(Some(result)) => match result {
                Ok(Message::Binary(media)) => Poll::Ready(Some(media)),
                Ok(Message::Close(_)) => Poll::Ready(None),
                _ => Poll::Pending,
            },
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
