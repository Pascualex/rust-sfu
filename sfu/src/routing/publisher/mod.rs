#![allow(clippy::module_inception)]

pub use publisher::Publisher;
pub use publisher_loop::publisher_loop;
pub use publisher_message::PublisherMessage;

mod publisher;
mod publisher_loop;
mod publisher_message;

pub type PublisherAddress = tokio::sync::mpsc::Sender<PublisherMessage>;
pub type PublisherMailbox = tokio::sync::mpsc::Receiver<PublisherMessage>;
