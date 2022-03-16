use std::time::Duration;

use tokio::{
    select,
    time::{interval, Interval, MissedTickBehavior},
};

use super::{Sfu, SfuMailbox, SfuMessage};

pub async fn sfu_loop(mut sfu: Sfu, mut mailbox: SfuMailbox) {
    println!("SFU loop starts");

    let mut keepalive = interval(Duration::from_secs(1));
    keepalive.set_missed_tick_behavior(MissedTickBehavior::Delay);

    while let Some(message) = recv(&mut mailbox, &mut keepalive).await {
        match message {
            SfuMessage::CreatePublisher(publisher) => sfu.create_publisher(publisher).await,
            SfuMessage::CreateSubscriber(subscriber) => sfu.create_subscriber(subscriber).await,
            SfuMessage::Keepalive => sfu.keepalive(),
        }
    }

    println!("SFU loop stops");
}

async fn recv(mailbox: &mut SfuMailbox, keepalive: &mut Interval) -> Option<SfuMessage> {
    select! {
        message = mailbox.recv() => message,
        _ = keepalive.tick() => Some(SfuMessage::Keepalive),
    }
}
