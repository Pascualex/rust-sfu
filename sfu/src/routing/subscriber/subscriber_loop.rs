use std::time::Duration;

use tokio::{
    select,
    time::{interval, Interval, MissedTickBehavior},
};

use super::{Subscriber, SubscriberMailbox, SubscriberMessage};

pub async fn subscriber_loop(mut subscriber: Subscriber, mut mailbox: SubscriberMailbox) {
    println!("Subscriber loop starts");

    let mut keepalive = interval(Duration::from_secs(1));
    keepalive.set_missed_tick_behavior(MissedTickBehavior::Delay);

    while let Some(message) = recv(&mut mailbox, &mut keepalive).await {
        match message {
            SubscriberMessage::Subscription(publisher) => subscriber.subscribe(publisher).await,
            SubscriberMessage::Data(data) => {
                if subscriber.transport(data).await.is_err() {
                    break;
                }
            }
            SubscriberMessage::Keepalive => {
                if subscriber.keepalive().await.is_err() {
                    break;
                }
            }
        }
    }

    println!("Subscriber loop stops");
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
