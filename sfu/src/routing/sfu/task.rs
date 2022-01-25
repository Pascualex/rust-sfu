use std::time::Duration;

use tokio::{
    select,
    sync::mpsc::{self, Receiver},
    task,
    time::{interval, Interval},
};

use super::{Actor, Address, Message};

pub fn spawn() -> Address {
    let (sender, receiver) = mpsc::channel(100);
    let actor = Actor::new();
    task::spawn(task_loop(receiver, actor));
    Address::new(sender)
}

async fn task_loop(mut receiver: Receiver<Message>, mut actor: Actor) {
    println!("SFU task starts");

    let mut keepalive = interval(Duration::from_secs(1));

    while let Some(message) = recv(&mut receiver, &mut keepalive).await {
        match message {
            Message::CreatePublisher(publisher) => actor.create_publisher(publisher).await,
            Message::CreateSubscriber(subscriber) => actor.create_subscriber(subscriber).await,
            Message::Keepalive => actor.keepalive(),
        }
    }

    println!("SFU task stops");
}

async fn recv(receiver: &mut Receiver<Message>, keepalive: &mut Interval) -> Option<Message> {
    select! {
        message = receiver.recv() => message,
        _ = keepalive.tick() => Some(Message::Keepalive),
    }
}
