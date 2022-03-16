use crate::{
    routing::publisher::{PublisherAddress, PublisherMessage},
    transport::{Data, DataSender, TransportError},
};

use super::SubscriberAddress;

pub struct Subscriber {
    address: SubscriberAddress,
    data_sndr: DataSender,
    num_subs: u32,
}

impl Subscriber {
    pub fn new(address: SubscriberAddress, data_sndr: DataSender) -> Self {
        Self {
            address,
            data_sndr,
            num_subs: 0,
        }
    }

    pub async fn subscribe(&mut self, publisher_addr: PublisherAddress) {
        if self.num_subs < 49 {
            let message = PublisherMessage::Subscription(self.address.clone());
            publisher_addr.send(message).await.ok(); // todo
            self.num_subs += 1;
        }
    }

    pub async fn transport(&mut self, data: Data) -> Result<(), TransportError> {
        self.data_sndr.send(data).await
    }

    pub async fn keepalive(&mut self) -> Result<(), TransportError> {
        self.data_sndr.keepalive().await
    }
}
