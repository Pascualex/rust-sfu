import "./Participant.css";

interface ParticipantProps {
  track: MediaStreamTrack | null,
  mirrored: boolean,
}

function Participant(props: ParticipantProps) {
  if (props.track === null) {
    return <div className="track placeholder">
      No device selected
    </div>;
  }
  const track = props.track;
  const ref = (video: HTMLVideoElement | null) => {
    if (video === null) return;
    video.srcObject = new MediaStream([track]);
  };
  const mirrored = props.mirrored ? " mirrored" : "";
  return <video className={"track" + mirrored} playsInline autoPlay ref={ref}></video>;
}

export default Participant;
