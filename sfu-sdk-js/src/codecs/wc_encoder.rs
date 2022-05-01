use async_trait::async_trait;
use flume::{Receiver, Sender};
use js_sys::{Error, Reflect};
use sfu_client::codecs::{Chunk, Encoder};
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{
    EncodedVideoChunk, EncodedVideoChunkMetadata, EncodedVideoChunkType, MediaStreamTrack,
    MediaStreamTrackProcessor, MediaStreamTrackProcessorInit, ReadableStreamDefaultReader,
    VideoEncoder, VideoEncoderConfig, VideoEncoderEncodeOptions, VideoEncoderInit, VideoFrame,
};

pub struct WcEncoder {
    receiver: Receiver<Chunk>,
}

impl WcEncoder {
    pub fn new(track: MediaStreamTrack) -> Self {
        let (sender, receiver) = flume::bounded(100);
        let width = Reflect::get(&track.get_settings(), &"width".into()).unwrap(); // todo
        let width = width.as_f64().unwrap() as u32; // todo
        let height = Reflect::get(&track.get_settings(), &"height".into()).unwrap(); // todo
        let height = height.as_f64().unwrap() as u32; // todo
        let encoder = Self::setup_encoder(width, height, sender);
        let processor = Self::setup_track_processor(track);
        spawn_local(Self::move_to_encoder(processor, encoder));
        Self { receiver }
    }

    fn setup_encoder(width: u32, height: u32, sender: Sender<Chunk>) -> VideoEncoder {
        let output = Closure::wrap(Box::new(
            move |c: EncodedVideoChunk, _: EncodedVideoChunkMetadata| {
                let is_key = c.type_() == EncodedVideoChunkType::Key;
                let mut bytes = vec![0; c.byte_length() as usize];
                c.copy_to_with_u8_array(&mut bytes);
                let chunk = Chunk::new(is_key, c.timestamp(), bytes);
                sender.send(chunk).unwrap(); // todo
            },
        )
            as Box<dyn FnMut(EncodedVideoChunk, EncodedVideoChunkMetadata)>);
        let error = Closure::wrap(Box::new(move |_| {}) as Box<dyn FnMut(Error)>);

        let output_ref = output.as_ref().unchecked_ref();
        let error_ref = error.as_ref().unchecked_ref();
        let init = VideoEncoderInit::new(error_ref, output_ref);
        output.forget(); // todo: this might leak memory
        error.forget(); // todo: this might leak memory

        let encoder = VideoEncoder::new(&init).unwrap(); // todo

        let mut config = VideoEncoderConfig::new("av01.0.01M.08", height, width);
        config.framerate(24.0);
        encoder.configure(&config);

        encoder
    }

    fn setup_track_processor(track: MediaStreamTrack) -> MediaStreamTrackProcessor {
        let init = MediaStreamTrackProcessorInit::new(&track);
        MediaStreamTrackProcessor::new(&init).unwrap() // todo
    }

    async fn move_to_encoder(processor: MediaStreamTrackProcessor, encoder: VideoEncoder) {
        let reader = ReadableStreamDefaultReader::new(&processor.readable()).unwrap(); // todo
        let mut frame_counter = 0;
        loop {
            let result = JsFuture::from(reader.read()).await.unwrap(); // todo
            let done = Reflect::get(&result, &"done".into()).unwrap(); // todo
            let done = done.as_bool().unwrap(); // todo
            let frame = Reflect::get(&result, &"value".into()).unwrap(); // todo
            let frame = frame.unchecked_into::<VideoFrame>();

            if done {
                break;
            }

            if encoder.encode_queue_size() > 2 {
                // Too many frames in flight, encoder is overwhelmed
                // let's drop this frame.
                frame.close();
            } else {
                frame_counter += 1;
                let mut options = VideoEncoderEncodeOptions::new();
                options.key_frame((frame_counter % 150) == 0);
                encoder.encode_with_options(&frame, &options);
                frame.close();
            }
        }
    }
}

#[async_trait]
impl Encoder for WcEncoder {
    async fn encode(&mut self) -> Option<Chunk> {
        self.receiver.recv_async().await.ok()
    }
}
