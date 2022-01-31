use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use futures_util::{stream::SplitStream, Stream, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use super::Data;

pub struct Producer {
    split_stream: SplitStream<WebSocketStream<TcpStream>>,
}

impl Producer {
    pub fn new(split_stream: SplitStream<WebSocketStream<TcpStream>>) -> Self {
        Self { split_stream }
    }
}

impl Stream for Producer {
    type Item = Data;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.split_stream.poll_next_unpin(cx) {
            Poll::Ready(Some(result)) => match result {
                Ok(Message::Binary(data)) => Poll::Ready(Some(Arc::new(data))),
                Ok(Message::Close(_)) => Poll::Ready(None),
                _ => {
                    cx.waker().wake_by_ref();
                    Poll::Pending
                },
            },
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
