use js_sys::{Error, Uint8Array};
use sfu_client::codecs::{Chunk, Decoder};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{
    EncodedVideoChunk, EncodedVideoChunkInit, EncodedVideoChunkType, MediaStreamTrack,
    MediaStreamTrackGenerator, MediaStreamTrackGeneratorInit, VideoDecoder, VideoDecoderConfig,
    VideoDecoderInit, VideoFrame,
};

pub struct WcDecoder {
    decoder: VideoDecoder,
    track: MediaStreamTrack,
    has_key: bool,
}

impl WcDecoder {
    fn setup_generator() -> MediaStreamTrackGenerator {
        let init = MediaStreamTrackGeneratorInit::new("video");
        MediaStreamTrackGenerator::new(&init).unwrap() // todo
    }

    fn setup_decoder(generator: &MediaStreamTrackGenerator) -> VideoDecoder {
        let writer = generator.writable().get_writer();

        let output = Closure::wrap(Box::new(move |f: VideoFrame| {
            let _ = writer.write_with_chunk(&f);
        }) as Box<dyn FnMut(VideoFrame)>);
        let error = Closure::wrap(Box::new(move |_| {}) as Box<dyn FnMut(Error)>);

        let output_ref = output.as_ref().unchecked_ref();
        let error_ref = error.as_ref().unchecked_ref();
        let init = VideoDecoderInit::new(error_ref, output_ref);
        output.forget(); // todo: this might leak memory
        error.forget(); // todo: this might leak memory

        let decoder = VideoDecoder::new(&init).unwrap(); // todo

        let config = VideoDecoderConfig::new("vp8");
        // config.coded_width(1280);
        // config.coded_height(720);
        decoder.configure(&config);

        decoder
    }
}

impl Decoder for WcDecoder {
    type Track = MediaStreamTrack;

    fn new() -> Self {
        let generator = Self::setup_generator();
        let decoder = Self::setup_decoder(&generator);

        Self {
            decoder,
            track: generator.into(),
            has_key: false,
        }
    }

    fn get_track(&self) -> Self::Track {
        self.track.clone()
    }

    fn decode(&mut self, chunk: Chunk) {
        let is_key = match chunk.is_key {
            true => EncodedVideoChunkType::Key,
            false => match self.has_key {
                true => EncodedVideoChunkType::Delta,
                false => return,
            },
        };
        self.has_key = true;
        let data = Uint8Array::from(&chunk.bytes[..]);
        let init = EncodedVideoChunkInit::new(&data, chunk.timestamp, is_key);
        let chunk = EncodedVideoChunk::new(&init).unwrap(); // todo
        self.decoder.decode(&chunk);
    }
}
