use uuid::Uuid;

use crate::{codecs::Encoder, endpoints::{PublisherEndpoint}};

use super::Publisher;

pub async fn publisher_loop<E: PublisherEndpoint>(
    mut publisher: Publisher<E>,
    mut encoder: impl Encoder,
) {
    let track_id = Uuid::new_v4();
    while let Some(chunk) = encoder.encode().await {
        publisher.send(track_id, chunk);
    }
}
