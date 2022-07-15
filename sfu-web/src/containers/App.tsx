import { useEffect, useState } from "react";
import init from "sfu-sdk-js";
import JoinMenu from "../components/JoinMenu";
import ParticipantGrid from "../components/ParticipantGrid";
import TrackSettings from "../components/TrackSettings";
import "./App.css";

function App() {
  const [localTrack, setLocalTrack] = useState<MediaStreamTrack | null>(null);
  const [remoteTracks, setRemoteTracks] = useState<MediaStreamTrack[]>([]);
  const [connected, setConnected] = useState<boolean>(false);
  const loaded = useLoadWasm();
  return <div className="background">
    <h1 className="web-title">Unbundled RTC</h1>
    <div className="center">
    {loaded && !connected ? <>
      <JoinMenu
        localTrack={localTrack}
        setConnected={setConnected}
        setRemoteTracks={setRemoteTracks}></JoinMenu>
      <TrackSettings setTrack={setLocalTrack}></TrackSettings>
    </> : null}
    {connected ? <>
      <ParticipantGrid localTrack={localTrack} remoteTracks={remoteTracks}></ParticipantGrid>
    </> : null}
    </div>
  </div>;
}

function useLoadWasm(): boolean {
  const [loaded, setLoaded] = useState<boolean>(false);
  useEffect(() => {
    init().then(() => setLoaded(true));
  }, []);
  return loaded;
}

export default App;
