use std::time::Duration;

use tokio::{
    select,
    time::{interval, Interval, MissedTickBehavior},
};

use crate::{endpoints::PublisherEndpoint, routing::KEEPALIVE_INTERVAL};

use super::{Publisher, PublisherMailbox, PublisherMessage};

pub async fn publisher_loop(
    mut publisher: Publisher,
    mut mailbox: PublisherMailbox,
    mut endpoint: PublisherEndpoint,
) {
    println!("Publisher loop starts ({})", publisher.id);

    let mut keepalive = interval(Duration::from_secs_f32(KEEPALIVE_INTERVAL));
    keepalive.set_missed_tick_behavior(MissedTickBehavior::Delay);

    while let Some(message) = recv(&mut mailbox, &mut endpoint, &mut keepalive).await {
        match message {
            PublisherMessage::Data(id, d) => publisher.route(id, d).await,
            PublisherMessage::Keepalive => publisher.keepalive(),
            PublisherMessage::Stop => break,
        }
    }

    publisher.stop().await;

    println!("Publisher loop stops ({})", publisher.id);
}

async fn recv(
    mailbox: &mut PublisherMailbox,
    endpoint: &mut PublisherEndpoint,
    keepalive: &mut Interval,
) -> Option<PublisherMessage> {
    select! {
        message = mailbox.recv() => message,
        data = endpoint.recv() => data.map(|(id, d)| PublisherMessage::Data(id, d)),
        _ = keepalive.tick() => Some(PublisherMessage::Keepalive),
    }
}
