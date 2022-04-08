use futures_util::{stream::SplitSink, SinkExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    tungstenite::{self, Message},
    WebSocketStream,
};
use uuid::Uuid;

use super::{Data, EndpointError};

pub struct SubscriberEndpoint {
    split_sink: SplitSink<WebSocketStream<TcpStream>, Message>,
}

impl SubscriberEndpoint {
    pub fn new(split_sink: SplitSink<WebSocketStream<TcpStream>, Message>) -> Self {
        Self { split_sink }
    }

    pub async fn send(&mut self, track_id: Uuid, data: Data) -> Result<(), EndpointError> {
        let data = (*data).clone();
        let binary = bincode::serialize(&(track_id, data)).unwrap();
        self.send_message(Message::Binary(binary)).await
    }

    pub async fn keepalive(&mut self) -> Result<(), EndpointError> {
        self.send_message(Message::Ping(vec![])).await
    }

    async fn send_message(&mut self, message: Message) -> Result<(), EndpointError> {
        match self.split_sink.send(message).await {
            Ok(()) => Ok(()),
            Err(tungstenite::Error::SendQueueFull(_)) => {
                println!("Send queue full");
                Ok(())
            }
            Err(_) => Err(EndpointError),
        }
    }
}
