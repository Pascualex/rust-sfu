use std::time::Duration;

use tokio::{
    select,
    time::{interval, Interval, MissedTickBehavior},
};

use crate::transport::DataReceiver;

use super::{Publisher, PublisherMailbox, PublisherMessage};

pub async fn publisher_loop(
    mut publisher: Publisher,
    mut mailbox: PublisherMailbox,
    mut data_rcvr: DataReceiver,
) {
    println!("Publisher loop starts");

    let mut keepalive = interval(Duration::from_secs(1));
    keepalive.set_missed_tick_behavior(MissedTickBehavior::Delay);

    while let Some(message) = recv(&mut mailbox, &mut data_rcvr, &mut keepalive).await {
        match message {
            PublisherMessage::Subscription(subscriber) => publisher.add_subscriber(subscriber),
            PublisherMessage::Data(data) => publisher.publish(data).await,
            PublisherMessage::Keepalive => publisher.keepalive(),
        }
    }

    println!("Publisher loop stops");
}

async fn recv(
    mailbox: &mut PublisherMailbox,
    data_rcvr: &mut DataReceiver,
    keepalive: &mut Interval,
) -> Option<PublisherMessage> {
    select! {
        message = mailbox.recv() => message,
        data = data_rcvr.recv() => data.map(PublisherMessage::Data),
        _ = keepalive.tick() => Some(PublisherMessage::Keepalive),
    }
}
