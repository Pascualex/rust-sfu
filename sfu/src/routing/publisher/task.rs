use std::time::Duration;

use futures_util::StreamExt;
use tokio::{
    select,
    sync::mpsc::{self, Receiver},
    task,
    time::{interval, Interval},
};

use crate::{routing::Subscriber, transport::MediaProducer};

use super::{Actor, Address, Message};

pub fn spawn(producer: MediaProducer, subscribers: Vec<Subscriber>) -> Address {
    let (sender, receiver) = mpsc::channel(100);
    let actor = Actor::new(subscribers);
    task::spawn(task_loop(receiver, producer, actor));
    Address::new(sender)
}

async fn task_loop(mut receiver: Receiver<Message>, mut producer: MediaProducer, mut actor: Actor) {
    println!("Publisher task starts");

    let mut keepalive = interval(Duration::from_secs(1));

    while let Some(message) = recv(&mut receiver, &mut producer, &mut keepalive).await {
        match message {
            Message::Subscription(subscriber) => actor.subscribe(subscriber),
            Message::Media(media) => actor.forward(media).await,
            Message::Keepalive => actor.keepalive(),
        }
    }

    println!("Publisher task stops");
}

async fn recv(
    receiver: &mut Receiver<Message>,
    producer: &mut MediaProducer,
    keepalive: &mut Interval,
) -> Option<Message> {
    select! {
        message = receiver.recv() => message,
        media = producer.next() => media.map(|m| Message::Media(m)),
        _ = keepalive.tick() => Some(Message::Keepalive),
    }
}
