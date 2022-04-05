use std::net::SocketAddr;

use futures_util::StreamExt;
use tokio::{net::TcpListener, sync::mpsc};
use uuid::Uuid;

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

    let actor = Sfu::new();
    let (address, mailbox) = mpsc::channel(100);
    tokio::task::spawn(sfu_loop(actor, mailbox));

    let sfu = address;

    println!("Listening on port {}", socket_addr.port());
    while let Ok((tcp_stream, incoming_addr)) = listener.accept().await {
        let ws_stream = tokio_tungstenite::accept_async(tcp_stream).await.unwrap();

        println!("WebSocket connection established with: {}", incoming_addr);

        let (ws_sink, ws_stream) = ws_stream.split();
        let receiver = DataReceiver::new(ws_stream);
        let sender = DataSender::new(ws_sink);

        let id = Uuid::new_v4();

        let message = SfuMessage::CreatePublisher(id, receiver);
        sfu.send(message).await.ok(); // todo

        let message = SfuMessage::CreateSubscriber(id, sender);
        sfu.send(message).await.ok(); // todo
    }

    let message = SfuMessage::Stop;
    sfu.send(message).await.ok(); // todo
}
