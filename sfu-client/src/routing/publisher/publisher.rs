use uuid::Uuid;

use crate::{endpoints::PublisherEndpoint, codecs::Chunk};

pub struct Publisher<E: PublisherEndpoint> {
    endpoint: E,
}

impl<E: PublisherEndpoint> Publisher<E> {
    pub fn new(endpoint: E) -> Self {
        Self { endpoint }
    }

    pub fn send(&mut self, id: Uuid, chunk: Chunk) {
        let data = bincode::serialize(&chunk).unwrap();
        let data = bincode::serialize(&(id, data)).unwrap();
        self.endpoint.send(data);
    }
}
