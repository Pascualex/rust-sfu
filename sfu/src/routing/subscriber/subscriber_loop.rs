use std::time::Duration;

use tokio::{
    select,
    time::{interval, Interval, MissedTickBehavior},
};

use crate::routing::KEEPALIVE_INTERVAL;

use super::{Subscriber, SubscriberMailbox, SubscriberMessage};

pub async fn subscriber_loop(mut subscriber: Subscriber, mut mailbox: SubscriberMailbox) {
    println!("Subscriber loop starts ({})", subscriber.id);

    let mut keepalive = interval(Duration::from_secs_f32(KEEPALIVE_INTERVAL));
    keepalive.set_missed_tick_behavior(MissedTickBehavior::Delay);

    while let Some(message) = recv(&mut mailbox, &mut keepalive).await {
        match message {
            SubscriberMessage::Data(id, d) => match subscriber.send(id, d).await {
                Ok(_) => (),
                Err(_) => break,
            },
            SubscriberMessage::Keepalive => match subscriber.keepalive().await {
                Ok(_) => (),
                Err(_) => break,
            },
            SubscriberMessage::Stop => break,
        }
    }

    println!("Subscriber loop stops ({})", subscriber.id);
}

async fn recv(
    mailbox: &mut SubscriberMailbox,
    keepalive: &mut Interval,
) -> Option<SubscriberMessage> {
    select! {
        message = mailbox.recv() => message,
        _ = keepalive.tick() => Some(SubscriberMessage::Keepalive),
    }
}
