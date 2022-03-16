use futures_util::{stream::SplitSink, SinkExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    tungstenite::{self, Message},
    WebSocketStream,
};

use super::{Data, TransportError};

pub struct DataSender {
    split_sink: SplitSink<WebSocketStream<TcpStream>, Message>,
}

impl DataSender {
    pub fn new(split_sink: SplitSink<WebSocketStream<TcpStream>, Message>) -> Self {
        Self { split_sink }
    }

    pub async fn send(&mut self, data: Data) -> Result<(), TransportError> {
        self.send_message(Message::Binary((*data).clone())).await
    }

    pub async fn keepalive(&mut self) -> Result<(), TransportError> {
        self.send_message(Message::Ping(vec![])).await
    }

    async fn send_message(&mut self, message: Message) -> Result<(), TransportError> {
        match self.split_sink.send(message).await {
            Ok(()) => Ok(()),
            Err(tungstenite::Error::SendQueueFull(_)) => {
                println!("Send queue full");
                Ok(())
            }
            Err(_) => Err(TransportError),
        }
    }
}
