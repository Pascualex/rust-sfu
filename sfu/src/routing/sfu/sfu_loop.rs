use std::time::Duration;

use tokio::{
    select,
    time::{interval, Interval, MissedTickBehavior},
};

use crate::routing::KEEPALIVE_INTERVAL;

use super::{Sfu, SfuMailbox, SfuMessage};

pub async fn sfu_loop(mut sfu: Sfu, mut mailbox: SfuMailbox) {
    println!("SFU loop starts");

    let mut keepalive = interval(Duration::from_secs_f32(KEEPALIVE_INTERVAL));
    keepalive.set_missed_tick_behavior(MissedTickBehavior::Delay);

    while let Some(message) = recv(&mut mailbox, &mut keepalive).await {
        match message {
            SfuMessage::CreatePublisher(r) => sfu.create_publisher(r).await,
            SfuMessage::CreateSubscriber(s) => sfu.create_subscriber(s).await,
            SfuMessage::Keepalive => sfu.keepalive(),
            SfuMessage::Stop => break,
        }
    }

    sfu.stop().await;

    println!("SFU loop stops");
}

async fn recv(mailbox: &mut SfuMailbox, keepalive: &mut Interval) -> Option<SfuMessage> {
    select! {
        message = mailbox.recv() => message,
        _ = keepalive.tick() => Some(SfuMessage::Keepalive),
    }
}
