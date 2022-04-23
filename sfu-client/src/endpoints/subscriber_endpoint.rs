use async_trait::async_trait;

#[async_trait]
pub trait SubscriberEndpoint {
    async fn recv(&mut self) -> Option<Vec<u8>>;
}
