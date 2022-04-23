use sfu_client::endpoints::PublisherEndpoint;
use web_sys::WebSocket;

pub struct WsPublisherEndpoint {
    ws: WebSocket,
}

impl WsPublisherEndpoint {
    pub fn new(ws: WebSocket) -> Self {
        Self { ws }
    }
}

impl PublisherEndpoint for WsPublisherEndpoint {
    fn send(&mut self, data: Vec<u8>) {
        self.ws.send_with_u8_array(&data).ok(); // todo
    }
}
