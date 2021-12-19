window.onload = start;

async function start() {
    const track = await getWebcamTrack();
    displayWebcamStream(track);
}

async function getWebcamTrack(): Promise<MediaStreamVideoTrack> {
    // const devicesInfo = await navigator.mediaDevices.enumerateDevices();
    // const deviceId = devicesInfo.find(x => x.label == "DroidCam Source 3")?.deviceId;
    // if (!deviceId) console.error("DroidCam Source 3 not found");
    const stream = await navigator.mediaDevices.getDisplayMedia({
        video: {
            width: 1720,
            height: 720,
            frameRate: 24,
            // deviceId: { exact: deviceId },
        },
    });
    return stream.getVideoTracks()[0];
}

function displayWebcamStream(track: MediaStreamVideoTrack) {
    const trackGenerator = new MediaStreamTrackGenerator({ kind: "video" });
    const trackGeneratorWriter = trackGenerator.writable.getWriter();

    const decoder = new VideoDecoder({
        output: videoFrame => trackGeneratorWriter.write(videoFrame),
        error: e => console.error(e),
    });
    decoder.configure({
        codec: "vp8",
        codedWidth: 1720,
        codedHeight: 720
    });
    
    const encoder = new VideoEncoder({
        output: (chunk, metadata) => decoder.decode(chunk),
        error: e => console.error(e),
    });
    encoder.configure({
        codec: "vp8",
        width: 1720,
        height: 720,
        framerate: 30,
    });

    const trackProcessor = new MediaStreamTrackProcessor({ track: track });
    const trackProcessorReader = trackProcessor.readable.getReader();

    moveToEncoder(trackProcessorReader, encoder);

    const outboundVideo = document.getElementById("outbound-video") as HTMLVideoElement;
    outboundVideo.srcObject = new MediaStream([track]);

    const inboundVideo = document.getElementById("inbound-video") as HTMLVideoElement;
    inboundVideo.srcObject = new MediaStream([trackGenerator]);
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