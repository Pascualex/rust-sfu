window.onload = start;

async function start() {
    const outboundVideo = document.getElementById("outbound-video") as HTMLVideoElement;
    const inboundVideo = document.getElementById("inbound-video") as HTMLVideoElement;

    const outboundTrack = await getLocalTrack();
    outboundVideo.srcObject = new MediaStream([outboundTrack]);
    const ws = new WebSocket("ws://localhost:8085")
    const encoderConfig = sendOutboundTrack(outboundTrack, ws);
    const inboundTrack = receiveInboundTrack(ws, encoderConfig);
    inboundVideo.srcObject = new MediaStream([inboundTrack]);
}

async function getLocalTrack(): Promise<MediaStreamVideoTrack> {
    const devicesInfo = await navigator.mediaDevices.enumerateDevices();
    const deviceId = devicesInfo.find(x => x.label == "DroidCam Source 3")?.deviceId;
    if (!deviceId) console.error("DroidCam Source 3 not found");
    const stream = await navigator.mediaDevices.getUserMedia({
        video: {
            width: 1280,
            height: 720,
            frameRate: 24,
            deviceId: { exact: deviceId },
        },
    });
    return stream.getVideoTracks()[0];
}

function sendOutboundTrack(track: MediaStreamVideoTrack, ws: WebSocket): VideoEncoderConfig {
    const encoder = new VideoEncoder({
        output: (chunk, metadata) => {
            const metadataBytes = new ArrayBuffer(5);
            const metadataView = new DataView(metadataBytes);
            metadataView.setInt8(0, chunk.type == 'key' ? 1 : 0);
            metadataView.setInt32(1, chunk.timestamp);
            const dataBytes = new Uint8Array(chunk.byteLength);
            chunk.copyTo(dataBytes);
            ws.send(new Blob([metadataBytes, dataBytes]));
        },
        error: e => console.error(e),
    });

    const settings = track.getSettings();
    const encoderConfig: VideoEncoderConfig = {
        codec: "vp09.00.41.08",
        width: settings.width!,
        height: settings.height!,
        framerate: settings.frameRate!,
        // scalabilityMode: "L1T3",
    };
    encoder.configure(encoderConfig);

    const trackProcessor = new MediaStreamTrackProcessor({ track: track });
    const trackProcessorReader = trackProcessor.readable.getReader();

    moveToEncoder(trackProcessorReader, encoder);

    return encoderConfig;
}

function receiveInboundTrack(ws: WebSocket, config: VideoEncoderConfig): MediaStreamVideoTrack {
    const trackGenerator = new MediaStreamTrackGenerator({ kind: "video" });
    const trackGeneratorWriter = trackGenerator.writable.getWriter();

    const decoder = new VideoDecoder({
        output: videoFrame => trackGeneratorWriter.write(videoFrame),
        error: e => console.error(e),
    });
    decoder.configure({
        codec: config.codec,
        codedWidth: config.width,
        codedHeight: config.height,
    });

    ws.onmessage = async ev => {
        const blob: Blob = ev.data;
        const bytes = await blob.arrayBuffer();
        const bytesView = new DataView(bytes);
        const isKey = bytesView.getInt8(0);
        const timestamp = bytesView.getUint32(1);
        const dataBytes = bytes.slice(5);
        decoder.decode(new EncodedVideoChunk({
            type: isKey ? "key" : "delta",
            timestamp: timestamp,
            data: dataBytes,
        }));
    };

    return trackGenerator;
}

async function moveToEncoder(reader: ReadableStreamDefaultReader, encoder: VideoEncoder) {
    let frame_counter = 0;
    while (true) {
        const result = await reader.read();
        if (result.done) break;
    
        let frame = result.value;
        if (encoder.encodeQueueSize > 2) {
            // Too many frames in flight, encoder is overwhelmed
            // let's drop this frame.
            console.log("Frame dropped");
            frame.close();
        } else {
            frame_counter++;
            const insert_keyframe = (frame_counter % 150) == 0;
            encoder.encode(frame, { keyFrame: insert_keyframe });
            frame.close();
        }
    }
}