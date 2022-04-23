use async_trait::async_trait;
use flume::{Receiver, Sender};
use js_sys::Uint8Array;
use sfu_client::endpoints::SubscriberEndpoint;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{BinaryType, MessageEvent, WebSocket};

pub struct WsSubscriberEndpoint {
    receiver: Receiver<Vec<u8>>,
}

impl WsSubscriberEndpoint {
    pub fn new(ws: WebSocket) -> Self {
        let (sender, receiver) = flume::bounded(100);
        Self::setup_ws_callbacks(ws, sender);
        Self { receiver }
    }

    pub fn setup_ws_callbacks(ws: WebSocket, sender: Sender<Vec<u8>>) {
        ws.set_binary_type(BinaryType::Arraybuffer);
        let onmessage = Closure::wrap(Box::new(move |e: MessageEvent| {
            let buffer = e.data();
            let buffer = buffer.as_ref().unchecked_ref();
            let data: Vec<u8> = Uint8Array::new(buffer).to_vec();
            sender.send(data).ok(); // todo
        }) as Box<dyn FnMut(MessageEvent)>);
        let onmessage_ref = &onmessage.as_ref().unchecked_ref();
        ws.set_onmessage(Some(onmessage_ref));
        onmessage.forget(); // todo: this might leak memory
    }
}

#[async_trait]
impl SubscriberEndpoint for WsSubscriberEndpoint {
    async fn recv(&mut self) -> Option<Vec<u8>> {
        self.receiver.recv_async().await.ok() // todo
    }
}
