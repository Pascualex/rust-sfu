use std::collections::{HashMap, hash_map::Entry};

use uuid::Uuid;

use crate::codecs::{Decoder};

pub struct Subscriber<D: Decoder> {
    decoders: HashMap<Uuid, D>,
    pub on_track: Box<dyn Fn(D::Track)>,
}

impl<D: Decoder> Subscriber<D> {
    pub fn new(on_track: Box<dyn Fn(D::Track)>) -> Self {
        Self {
            decoders: HashMap::new(),
            on_track,
        }
    }

    pub fn decode(&mut self, data: Vec<u8>) {
        let (track_id, data) = bincode::deserialize::<(Uuid, Vec<u8>)>(&data).unwrap();
        let chunk = bincode::deserialize(&data).unwrap();

        let decoder = match self.decoders.entry(track_id) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                let decoder = D::new();
                (self.on_track)(decoder.get_track());
                v.insert(decoder)
            }
        };

        decoder.decode(chunk);
    }
}
