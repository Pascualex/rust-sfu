pub use address::Address as Subscriber;
pub use task::spawn as spawn_subscriber;

mod actor;
mod address;
mod message;
mod task;

use actor::Actor;
use address::Address;
use message::Message;
