pub use subscription_manager::SubscriptionManager;
pub use subscription_manager_loop::subscription_manager_loop;
pub use subscription_manager_message::SubscriptionManagerMessage;

mod state;
mod subscription_manager;
mod subscription_manager_loop;
mod subscription_manager_message;

pub type SubscriptionManagerAddress = tokio::sync::mpsc::Sender<SubscriptionManagerMessage>;
type SubscriptionManagerMailbox = tokio::sync::mpsc::Receiver<SubscriptionManagerMessage>;
