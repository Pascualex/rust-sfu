use std::net::SocketAddr;

use futures_util::StreamExt;
use tokio::net::TcpListener;

use crate::{
    routing::{sfu_loop, Sfu, SfuMessage},
    transport::{DataReceiver, DataSender},
};

mod routing;
mod transport;

#[tokio::main(flavor = "current_thread")]
// #[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    let socket_addr = "[::]:8085".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(socket_addr).await.unwrap();

    let (address, mailbox) = tokio::sync::mpsc::channel(100);
    let publisher = Sfu::new();
    tokio::task::spawn(sfu_loop(publisher, mailbox));

    let sfu = address;

    println!("Listening on port {}", socket_addr.port());
    while let Ok((tcp_stream, incoming_addr)) = listener.accept().await {
        let ws_stream = tokio_tungstenite::accept_async(tcp_stream).await.unwrap();

        println!("WebSocket connection established with: {}", incoming_addr);

        let (ws_sink, ws_stream) = ws_stream.split();
        let receiver = DataReceiver::new(ws_stream);
        let sender = DataSender::new(ws_sink);

        let message = SfuMessage::CreatePublisher(receiver);
        sfu.send(message).await.ok(); // todo

        let message = SfuMessage::CreateSubscriber(sender);
        sfu.send(message).await.ok(); // todo
    }

    let message = SfuMessage::Stop;
    sfu.send(message).await.ok(); // todo
}
