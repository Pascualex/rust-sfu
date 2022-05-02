#![allow(clippy::module_inception)]

pub use router::Router;
pub use router_loop::router_loop;
pub use router_message::RouterMessage;

mod router;
mod router_loop;
mod router_message;

pub type RouterAddress = tokio::sync::mpsc::Sender<RouterMessage>;
type RouterMailbox = tokio::sync::mpsc::Receiver<RouterMessage>;
pub type RouterChannel = (RouterAddress, RouterMailbox);
