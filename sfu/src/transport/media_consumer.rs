use futures_util::{stream::SplitSink, SinkExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    tungstenite::{self, Message},
    WebSocketStream,
};

use super::{Media, TransportError};

pub struct MediaConsumer {
    split_sink: SplitSink<WebSocketStream<TcpStream>, Message>,
}

impl MediaConsumer {
    pub fn new(split_sink: SplitSink<WebSocketStream<TcpStream>, Message>) -> Self {
        Self { split_sink }
    }

    pub async fn send(&mut self, media: Media) -> Result<(), TransportError> {
        self.send_message(Message::Binary(media)).await
    }

    pub async fn keepalive(&mut self) -> Result<(), TransportError> {
        self.send_message(Message::Ping(vec![])).await
    }

    async fn send_message(&mut self, message: Message) -> Result<(), TransportError> {
        match self.split_sink.send(message).await {
            Ok(()) => Ok(()),
            Err(tungstenite::Error::ConnectionClosed) => Err(TransportError),
            Err(tungstenite::Error::AlreadyClosed) => Err(TransportError),
            Err(err) => {
                println!("{}", err);
                Ok(())
            }
        }
    }
}
