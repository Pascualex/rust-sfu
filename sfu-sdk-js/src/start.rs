use js_sys::Function;
use sfu_client::routing::{
    publisher::{publisher_loop, Publisher},
    subscriber::{subscriber_loop, Subscriber},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{MediaStreamTrack, WebSocket};

use crate::{
    codecs::{WcDecoder, WcEncoder},
    endpoints::{WsPublisherEndpoint, WsSubscriberEndpoint},
};

#[wasm_bindgen(js_name = startConnection)]
pub fn start_connection(ws: WebSocket, track: MediaStreamTrack, on_track: Function) {
    start_publisher(ws.clone(), track);
    start_subscriber(ws, on_track);
}

fn start_publisher(ws: WebSocket, track: MediaStreamTrack) {
    let encoder = WcEncoder::new(track);
    let endpoint = WsPublisherEndpoint::new(ws);
    let publisher = Publisher::new(endpoint);
    spawn_local(publisher_loop(publisher, encoder));
}

fn start_subscriber(ws: WebSocket, on_track: Function) {
    let endpoint = WsSubscriberEndpoint::new(ws);
    let subscriber = Subscriber::<WcDecoder>::new(Box::new(move |track| {
        on_track.call1(&JsValue::NULL, &track).ok(); // todo
    }));
    spawn_local(subscriber_loop(subscriber, endpoint));
}
