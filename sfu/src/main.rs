use std::net::SocketAddr;

use futures_util::StreamExt;
use tokio::net::TcpListener;

use crate::{
    routing::spawn_sfu,
    transport::{MediaConsumer, MediaProducer},
};

mod routing;
mod transport;

#[tokio::main]
async fn main() {
    let addr = "[::]:8085".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();

    let sfu = spawn_sfu();

    println!("Listening on: {}", addr);
    while let Ok((tcp_stream, incoming_addr)) = listener.accept().await {
        let ws_stream = tokio_tungstenite::accept_async(tcp_stream).await.unwrap();

        println!("WebSocket connection established with: {}", incoming_addr);

        let (split_sink, split_stream) = ws_stream.split();
        // todo: add identifiers to avoid reflection
        let producer = MediaProducer::new(split_stream);
        let consumer = MediaConsumer::new(split_sink);
        sfu.create_publisher(producer).await;
        sfu.create_subscriber(consumer).await;
    }
}
