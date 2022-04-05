pub use sfu::Sfu;
pub use sfu_loop::sfu_loop;
pub use sfu_message::SfuMessage;

mod info;
mod sfu;
mod sfu_loop;
mod sfu_message;

// pub type SfuAddress = tokio::sync::mpsc::Sender<SfuMessage>;
type SfuMailbox = tokio::sync::mpsc::Receiver<SfuMessage>;
