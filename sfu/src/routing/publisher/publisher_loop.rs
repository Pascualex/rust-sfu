use std::time::Duration;

use tokio::{
    select,
    time::{interval, Interval, MissedTickBehavior},
};

use crate::{routing::KEEPALIVE_INTERVAL, transport::DataReceiver};

use super::{Publisher, PublisherMailbox, PublisherMessage};

pub async fn publisher_loop(
    mut publisher: Publisher,
    mut mailbox: PublisherMailbox,
    mut receiver: DataReceiver,
) {
    println!("Publisher loop starts ({})", publisher.id);

    let mut keepalive = interval(Duration::from_secs_f32(KEEPALIVE_INTERVAL));
    keepalive.set_missed_tick_behavior(MissedTickBehavior::Delay);

    while let Some(message) = recv(&mut mailbox, &mut receiver, &mut keepalive).await {
        match message {
            PublisherMessage::Data(d) => publisher.route(d).await,
            PublisherMessage::Keepalive => publisher.keepalive(),
            PublisherMessage::Stop => break,
        }
    }

    publisher.stop().await;

    println!("Publisher loop stops ({})", publisher.id);
}

async fn recv(
    mailbox: &mut PublisherMailbox,
    receiver: &mut DataReceiver,
    keepalive: &mut Interval,
) -> Option<PublisherMessage> {
    select! {
        message = mailbox.recv() => message,
        data = receiver.recv() => data.map(PublisherMessage::Data),
        _ = keepalive.tick() => Some(PublisherMessage::Keepalive),
    }
}
