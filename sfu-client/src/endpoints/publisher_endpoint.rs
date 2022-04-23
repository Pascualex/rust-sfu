pub trait PublisherEndpoint {
    fn send(&mut self, data: Vec<u8>);
}
