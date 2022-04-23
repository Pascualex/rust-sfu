use async_trait::async_trait;

use super::Chunk;

#[async_trait]
pub trait Encoder {
    async fn encode(&mut self) -> Option<Chunk>;
}
