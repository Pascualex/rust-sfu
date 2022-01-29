use std::time::Duration;

use tokio::{
    select,
    sync::mpsc::{self, Receiver},
    task,
    time::{interval, Interval, MissedTickBehavior},
};

use crate::transport::Consumer;

use super::{Actor, Address, Message};

pub fn spawn(consumer: Consumer) -> Address {
    let (sender, receiver) = mpsc::channel(100);
    let actor = Actor::new(consumer);
    task::spawn(task_loop(receiver, actor));
    Address::new(sender)
}

async fn task_loop(mut receiver: Receiver<Message>, mut actor: Actor) {
    println!("Subscriber task starts");

    let mut keepalive = interval(Duration::from_secs(1));
    keepalive.set_missed_tick_behavior(MissedTickBehavior::Delay);

    while let Some(message) = recv(&mut receiver, &mut keepalive).await {
        match message {
            Message::Data(data) => {
                if actor.consume(data).await.is_err() {
                    break;
                }
            }
            Message::Keepalive => {
                if actor.keepalive().await.is_err() {
                    break;
                }
            }
        }
    }

    println!("Subscriber task stops");
}

async fn recv(receiver: &mut Receiver<Message>, keepalive: &mut Interval) -> Option<Message> {
    select! {
        message = receiver.recv() => message,
        _ = keepalive.tick() => Some(Message::Keepalive),
    }
}
