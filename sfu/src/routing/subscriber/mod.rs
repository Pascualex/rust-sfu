pub use subscriber::Subscriber;
pub use subscriber_loop::subscriber_loop;
pub use subscriber_message::SubscriberMessage;

mod subscriber;
mod subscriber_loop;
mod subscriber_message;

pub type SubscriberAddress = tokio::sync::mpsc::Sender<SubscriberMessage>;
pub type SubscriberMailbox = tokio::sync::mpsc::Receiver<SubscriberMessage>;
