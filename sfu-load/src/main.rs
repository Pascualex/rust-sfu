use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde::{Deserialize, Serialize};
use std::{
    cmp::min,
    io::{stdout, Write},
};
use tokio::{
    net::TcpStream,
    task::JoinHandle,
    time::{interval, Duration, Instant, MissedTickBehavior},
};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use url::Url;

type Consumer = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type Producer = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

// 150 -  7x7  -> 1 thread
// 250 -  7x7  -> 2 threads
// 100 - 10x10 -> 2 threads

#[tokio::main]
async fn main() {
    let clients = 150;
    let max_subs = 49;
    let duration = Duration::from_secs(30);
    let size = 30_000;
    let freq = 24;

    println!("Parameters:");
    println!("  - Clients: {}", clients);
    println!("  - Max subs: {}", max_subs);
    println!("  - Packet freq: {} Hz", freq);
    println!("  - Packet size: {}", format_bit(size));
    println!("  - Duration: {} s", duration.as_secs());

    let c_up = size * freq;
    let c_down = size * freq * min(clients, max_subs);
    let cs_up = c_up * clients;
    let cs_down = c_down * clients;

    println!("Data bandwidths:");
    println!("  - Client upstream: {}ps", format_bit(c_up));
    println!("  - Client downstream: {}ps", format_bit(c_down));
    println!("  - Clients upstream: {}ps", format_bit(cs_up));
    println!("  - Clients downstream: {}ps", format_bit(cs_down));

    let url = "ws://localhost:8085".parse().unwrap();
    let ws_streams = connect_clients(url, clients).await;

    let iter = (0..(size / 8)).map(|n| (n % u8::MAX as u64) as u8);
    let data = iter.collect::<Vec<u8>>();
    let (send_handles, recv_handles) = start_test(ws_streams, data, freq, duration).await;

    print_progress(duration).await;

    let (send_count, recv_count, rtt) = get_results(send_handles, recv_handles).await;
    let send_expected = duration.as_secs() as u64 * freq * clients;
    let send_percentage = (send_count as f64 / send_expected as f64) * 100f64;
    let recv_expected = send_expected * min(clients, max_subs);
    let recv_percentage = (recv_count as f64 / recv_expected as f64) * 100f64;

    println!("Test finished:");
    println!(
        "  - Send: {} / {} ({:.2}%)",
        send_count, send_expected, send_percentage
    );
    println!(
        "  - Recv: {} / {} ({:.2}%)",
        recv_count, recv_expected, recv_percentage
    );
    println!("  - RTT: {} ms", rtt);
}

async fn connect_clients(url: Url, count: u64) -> Vec<WebSocketStream<MaybeTlsStream<TcpStream>>> {
    let mut ws_streams = Vec::new();
    for _ in 0..count {
        let (ws_stream, _) = connect_async(&url).await.unwrap();
        ws_streams.push(ws_stream);
    }
    ws_streams
}

async fn start_test(
    ws_streams: Vec<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    data: Vec<u8>,
    freq: u64,
    duration: Duration,
) -> (Vec<JoinHandle<u64>>, Vec<JoinHandle<(u64, u128)>>) {
    let mut send_handles = Vec::new();
    let mut recv_handles = Vec::new();
    let total = duration.as_secs() as u64 * freq;
    let start = Instant::now();
    for ws_stream in ws_streams {
        let (consumer, producer) = ws_stream.split();
        let send_fut = send(consumer, data.clone(), freq, total, start, duration);
        send_handles.push(tokio::spawn(send_fut));
        let recv_fut = recv(producer, data.clone(), start, duration);
        recv_handles.push(tokio::spawn(recv_fut));
    }
    (send_handles, recv_handles)
}

async fn send(
    mut consumer: Consumer,
    data: Vec<u8>,
    freq: u64,
    total: u64,
    start: Instant,
    duration: Duration,
) -> u64 {
    let finish = start + duration + Duration::from_secs(1);
    let mut interval = interval(Duration::from_millis(1000 / freq as u64));
    interval.set_missed_tick_behavior(MissedTickBehavior::Burst);
    let mut count = 0;
    for _ in 0..total {
        if Instant::now() >= finish {
            break;
        }
        interval.tick().await;
        let packet = Packet::new(data.clone(), start.elapsed());
        let packet = bincode::serialize(&packet).unwrap();
        let message = Message::Binary(packet);
        if consumer.send(message).await.is_err() {
            break;
        }
        count += 1;
    }
    count
}

async fn recv(
    mut producer: Producer,
    data: Vec<u8>,
    start: Instant,
    duration: Duration,
) -> (u64, u128) {
    let finish = start + duration + Duration::from_secs(5);
    let mut count = 0;
    let mut total_rtt = 0;
    while let Some(result) = producer.next().await {
        if Instant::now() >= finish {
            break;
        }
        match result {
            Ok(Message::Binary(recv_packet)) => {
                let recv_packet = bincode::deserialize::<Packet>(&recv_packet).unwrap();
                let rtt = (start.elapsed() - recv_packet.duration).as_millis();
                if recv_packet.data != data {
                    panic!("Packet received is corrupted");
                }
                count += 1;
                total_rtt += rtt;
            }
            Err(_) => break,
            _ => (),
        }
    }
    (count, total_rtt)
}

async fn print_progress(duration: Duration) {
    let mut interval = interval(Duration::from_secs(1));
    interval.set_missed_tick_behavior(MissedTickBehavior::Burst);
    for i in 0..duration.as_secs() {
        interval.tick().await;
        print!("Running for {} of {} seconds\r", i + 1, duration.as_secs());
        stdout().flush().unwrap();
    }
    println!();
}

async fn get_results(
    send_handles: Vec<JoinHandle<u64>>,
    recv_handles: Vec<JoinHandle<(u64, u128)>>,
) -> (u64, u64, u128) {
    let mut send_count = 0;
    for send_handle in send_handles {
        send_count += send_handle.await.unwrap();
    }
    let mut recv_count = 0;
    let mut rtt = 0;
    for recv_handle in recv_handles {
        let recv_results = recv_handle.await.unwrap();
        recv_count += recv_results.0;
        rtt += recv_results.1;
    }
    (send_count, recv_count, rtt / recv_count as u128)
}

fn format_bit(size: u64) -> String {
    if size < 10_000 {
        format!("{} b", size)
    } else if size < 10_000_000 {
        format!("{} Kb", size / 1_000)
    } else if size < 10_000_000_000 {
        format!("{} Mb", size / 1_000_000)
    } else {
        format!("{} Gb", size / 1_000_000_000)
    }
}

#[derive(Serialize, Deserialize)]
struct Packet {
    data: Vec<u8>,
    duration: Duration,
}

impl Packet {
    pub fn new(data: Vec<u8>, duration: Duration) -> Self {
        Self { data, duration }
    }
}
