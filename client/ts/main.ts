window.onload = start;

async function start() {
    const stream = await getWebcamStream();
    displayWebcamStream(stream);
    generateOffer(stream);
}

async function getWebcamStream() {
    const devicesInfo = await navigator.mediaDevices.enumerateDevices();
    console.log(devicesInfo);
    const deviceId = devicesInfo.find(x => x.label == "DroidCam Source 3")?.deviceId;
    if (!deviceId) console.error("DroidCam Source 3 not found");
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
    const video = document.getElementById("outbound-video") as HTMLVideoElement;
    video.srcObject = stream;
}

async function generateOffer(stream: MediaStream) {
    const pc = new RTCPeerConnection({
        iceServers: [{ urls: 'stun:stun.l.google.com:19302' }]
    });

    const video = document.getElementById("inbound-video") as HTMLVideoElement;
    pc.ontrack = ev => video.srcObject = ev.streams[0];

    pc.addTransceiver(stream.getVideoTracks()[0], {
        direction: "sendonly",
        streams: [stream],
        // sendEncodings: [
        //     { rid: "h", scaleResolutionDownBy: 1 },
        //     { rid: "m", scaleResolutionDownBy: 2 },
        //     { rid: "l", scaleResolutionDownBy: 4 },
        // ],
    });

    const offer = await pc.createOffer({
        // offerToReceiveAudio: true,
        offerToReceiveVideo: true,
    });
    updateSPDCodecs(offer);
    await pc.setLocalDescription(offer);

    const offerField = document.getElementById("offer") as HTMLParagraphElement;
    offerField.textContent = btoa(JSON.stringify(pc.localDescription));

    const answerField = document.getElementById("answer") as HTMLTextAreaElement;
    const processAnswerButton = document.getElementById("process-answer") as HTMLButtonElement;
    processAnswerButton.onclick = _ => {
        const answer = new RTCSessionDescription(JSON.parse(atob(answerField.value)));
        pc.setRemoteDescription(answer);
    };
}

function updateSPDCodecs(desc: RTCSessionDescriptionInit) {
    if (desc.sdp == null) return;

    const lines = desc.sdp.split("\n");

    const idx = lines.findIndex(x => x.startsWith("m=video"));
    const words = lines[idx].split(" ");
    // words[3] = "35"; // av1
    // words[17] = "96"; // vp8
    lines[idx] = words.join(" ");

    desc.sdp = lines.join("\n");
}
