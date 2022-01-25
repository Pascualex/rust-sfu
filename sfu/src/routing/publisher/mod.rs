pub use address::Address as Publisher;
pub use task::spawn as spawn_publisher;

mod actor;
mod address;
mod message;
mod task;

use actor::Actor;
use address::Address;
use message::Message;
