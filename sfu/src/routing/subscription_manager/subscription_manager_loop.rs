use std::time::Duration;

use tokio::{
    select,
    time::{interval, Interval, MissedTickBehavior},
};

use crate::routing::KEEPALIVE_INTERVAL;

use super::{SubscriptionManager, SubscriptionManagerMailbox, SubscriptionManagerMessage};

pub async fn subscription_manager_loop(
    mut subscription_manager: SubscriptionManager,
    mut mailbox: SubscriptionManagerMailbox,
) {
    println!("SubscriptionManager loop starts");

    let mut keepalive = interval(Duration::from_secs_f32(KEEPALIVE_INTERVAL));
    keepalive.set_missed_tick_behavior(MissedTickBehavior::Delay);

    while let Some(message) = recv(&mut mailbox, &mut keepalive).await {
        match message {
            SubscriptionManagerMessage::CreateRouter(id, p_id, c) => {
                subscription_manager.create_router(id, p_id, c).await
            }
            SubscriptionManagerMessage::CreateSubscriber(id, s) => {
                subscription_manager.create_subscriber(id, s).await
            }
            SubscriptionManagerMessage::Keepalive => subscription_manager.keepalive(),
            SubscriptionManagerMessage::Stop => break,
        }
    }

    subscription_manager.stop().await;

    println!("SubscriptionManager loop stops");
}

async fn recv(
    mailbox: &mut SubscriptionManagerMailbox,
    keepalive: &mut Interval,
) -> Option<SubscriptionManagerMessage> {
    select! {
        message = mailbox.recv() => message,
        _ = keepalive.tick() => Some(SubscriptionManagerMessage::Keepalive),
    }
}
