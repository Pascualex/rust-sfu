pub use sfu::*;

mod board;
mod publisher;
mod router;
mod routing_error;
mod sfu;
mod subscriber;

pub const KEEPALIVE_INTERVAL: f32 = 1.0;
pub const MAX_SUBSCRIPTIONS: usize = 49;
