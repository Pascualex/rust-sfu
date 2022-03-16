use std::net::SocketAddr;

use futures_util::StreamExt;
use tokio::{net::TcpListener, sync::mpsc, task};

use crate::{
    routing::{sfu_loop, Sfu, SfuMessage},
    transport::{DataReceiver, DataSender},
};

mod routing;
mod transport;

#[tokio::main(flavor = "current_thread")]
// #[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    let addr = "[::]:8085".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();

    let (sfu_addr, sfu_mailbox) = mpsc::channel(100);
    let sfu = Sfu::new();
    task::spawn(sfu_loop(sfu, sfu_mailbox));

    println!("Listening on: {}", addr);
    while let Ok((tcp_stream, incoming_addr)) = listener.accept().await {
        let ws_stream = tokio_tungstenite::accept_async(tcp_stream).await.unwrap();

        println!("WebSocket connection established with: {}", incoming_addr);

        let (split_sink, split_stream) = ws_stream.split();
        // todo: add identifiers to avoid reflection
        let data_receiver = DataReceiver::new(split_stream);
        let data_sender = DataSender::new(split_sink);
        let publisher_message = SfuMessage::CreatePublisher(data_receiver);
        sfu_addr.send(publisher_message).await.ok(); // todo
        let subscriber_message = SfuMessage::CreateSubscriber(data_sender);
        sfu_addr.send(subscriber_message).await.ok(); // todo
    }
}
