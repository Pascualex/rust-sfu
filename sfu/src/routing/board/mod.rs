pub use board::Board;
pub use board_loop::board_loop;
pub use board_message::BoardMessage;

mod board;
mod board_loop;
mod board_message;
mod state;

pub type BoardAddress = tokio::sync::mpsc::Sender<BoardMessage>;
type BoardMailbox = tokio::sync::mpsc::Receiver<BoardMessage>;
