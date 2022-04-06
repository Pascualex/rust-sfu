use std::sync::Arc;

use futures_util::{stream::SplitStream, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use super::Data;

pub struct PublisherEndpoint {
    split_stream: SplitStream<WebSocketStream<TcpStream>>,
}

impl PublisherEndpoint {
    pub fn new(split_stream: SplitStream<WebSocketStream<TcpStream>>) -> Self {
        Self { split_stream }
    }

    pub async fn recv(&mut self) -> Option<Data> {
        loop {
            match self.split_stream.next().await {
                Some(result) => match result {
                    Ok(Message::Binary(data)) => return Some(Arc::new(data)),
                    Ok(Message::Close(_)) => return None,
                    _ => (),
                },
                None => return None,
            }
        }
    }
}
