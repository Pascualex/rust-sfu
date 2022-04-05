use std::time::Duration;

use tokio::{
    select,
    time::{interval, Interval, MissedTickBehavior},
};

use crate::routing::KEEPALIVE_INTERVAL;

use super::{Board, BoardMailbox, BoardMessage};

pub async fn board_loop(mut board: Board, mut mailbox: BoardMailbox) {
    println!("Board loop starts");

    let mut keepalive = interval(Duration::from_secs_f32(KEEPALIVE_INTERVAL));
    keepalive.set_missed_tick_behavior(MissedTickBehavior::Delay);

    while let Some(message) = recv(&mut mailbox, &mut keepalive).await {
        match message {
            BoardMessage::CreateRouter(id, p_id, c) => board.create_router(id, p_id, c).await,
            BoardMessage::CreateSubscriber(id, s) => board.create_subscriber(id, s).await,
            BoardMessage::Keepalive => board.keepalive(),
            BoardMessage::Stop => break,
        }
    }

    board.stop().await;

    println!("Board loop stops");
}

async fn recv(mailbox: &mut BoardMailbox, keepalive: &mut Interval) -> Option<BoardMessage> {
    select! {
        message = mailbox.recv() => message,
        _ = keepalive.tick() => Some(BoardMessage::Keepalive),
    }
}
