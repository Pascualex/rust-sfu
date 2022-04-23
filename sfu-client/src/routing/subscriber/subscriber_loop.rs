use crate::{codecs::Decoder, endpoints::{SubscriberEndpoint}};

use super::Subscriber;

pub async fn subscriber_loop<D: Decoder>(
    mut subscriber: Subscriber<D>,
    mut endpoint: impl SubscriberEndpoint,
) {
    while let Some(data) = endpoint.recv().await {
        subscriber.decode(data);
    }
}
