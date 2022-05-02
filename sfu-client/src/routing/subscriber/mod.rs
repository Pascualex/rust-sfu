#![allow(clippy::module_inception)]

pub use subscriber::Subscriber;
pub use subscriber_loop::subscriber_loop;

mod subscriber;
mod subscriber_loop;
