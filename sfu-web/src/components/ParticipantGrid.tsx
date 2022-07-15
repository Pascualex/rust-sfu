import Participant from "./Participant";
import "./ParticipantGrid.css";

interface ParticipantGridProps {
  localTrack: MediaStreamTrack | null,
  remoteTracks: MediaStreamTrack[],
}

function ParticipantGrid(props: ParticipantGridProps) {
  let participants = [];
  if (props.localTrack != null) {
    participants.push(buildParticipant(props.localTrack, false));
  }
  for (const remoteTrack of props.remoteTracks) {
    participants.push(buildParticipant(remoteTrack, false));
  }
  return <div className="participant-grid">
    {participants}
  </div>;
}

function buildParticipant(track: MediaStreamTrack, mirrored: boolean) {
  return <div className="grid-participant-wrapper" key={track.id}>
    <Participant track={track} mirrored={mirrored}></Participant>
  </div>;
}

export default ParticipantGrid;
