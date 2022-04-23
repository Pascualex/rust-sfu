use super::Chunk;

pub trait Decoder {
    type Track;

    fn new() -> Self;

    fn get_track(&self) -> Self::Track;

    fn decode(&mut self, chunk: Chunk);
}
