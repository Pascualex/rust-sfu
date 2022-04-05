use std::time::Duration;

use tokio::{
    select,
    time::{interval, Interval, MissedTickBehavior},
};

use crate::routing::KEEPALIVE_INTERVAL;

use super::{Router, RouterMailbox, RouterMessage};

pub async fn router_loop(mut router: Router, mut mailbox: RouterMailbox) {
    println!("Router loop starts ({})", router.id);

    let mut keepalive = interval(Duration::from_secs_f32(KEEPALIVE_INTERVAL));
    keepalive.set_missed_tick_behavior(MissedTickBehavior::Delay);

    while let Some(message) = recv(&mut mailbox, &mut keepalive).await {
        match message {
            RouterMessage::Data(d) => router.route(d).await,
            RouterMessage::Subscriber(id, s) => router.add_subscriber(id, s),
            RouterMessage::Keepalive => router.keepalive().await,
            RouterMessage::Stop => break,
        }
    }

    println!("Router loop stops ({})", router.id);
}

async fn recv(mailbox: &mut RouterMailbox, keepalive: &mut Interval) -> Option<RouterMessage> {
    select! {
        message = mailbox.recv() => message,
        _ = keepalive.tick() => Some(RouterMessage::Keepalive),
    }
}
