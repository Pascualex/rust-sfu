window.onload = start;

async function start() {
    const stream = await getWebcamStream();
    displayWebcamStream(stream);
    startCall(stream);
}

async function getWebcamStream() {
    const devicesInfo = await navigator.mediaDevices.enumerateDevices();
    const deviceId = devicesInfo.find(x => x.label == "DroidCam Source 3")?.deviceId;
    return navigator.mediaDevices.getUserMedia({
        video: {
            width: 1280,
            height: 720,
            frameRate: 24,
            deviceId: { exact: deviceId },
        },
    });
}

function displayWebcamStream(stream: MediaStream) {
    const video = document.getElementById("sender-video") as HTMLVideoElement;
    video.srcObject = stream;
}

async function startCall(stream: MediaStream) {
    const spc = new RTCPeerConnection();
    const rpc = new RTCPeerConnection();

    spc.onicecandidate = e => rpc.addIceCandidate(e.candidate || undefined);
    rpc.onicecandidate = e => spc.addIceCandidate(e.candidate || undefined);
    const video = document.getElementById("receiver-video") as HTMLVideoElement;
    rpc.ontrack = e => video.srcObject = e.streams[0];

    spc.addTransceiver(stream.getVideoTracks()[0], {
        direction: "sendonly",
        streams: [stream],
    });

    const offer = await spc.createOffer({
        offerToReceiveAudio: true,
        offerToReceiveVideo: true,
    });
    updateSPDCodecs(offer);
    await spc.setLocalDescription(offer);
    await rpc.setRemoteDescription(offer);

    const answer = await rpc.createAnswer();
    updateSPDCodecs(answer);
    await rpc.setLocalDescription(answer);
    await spc.setRemoteDescription(answer);
}

function updateSPDCodecs(desc: RTCSessionDescriptionInit) {
    if (desc.sdp == null) return;

    const lines = desc.sdp.split("\n");

    const idx = lines.findIndex(x => x.startsWith("m=video"));
    const words = lines[idx].split(" ");
    words[3] = "35"; // av1
    words[17] = "96"; // vp8
    lines[idx] = words.join(" ");

    desc.sdp = lines.join("\n");
}
