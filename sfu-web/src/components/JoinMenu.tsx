import { useState } from "react";
import "./JoinMenu.css";
import { startConnection } from "sfu-sdk-js";

interface JoinMenuProps {
  localTrack: MediaStreamTrack | null,
  setConnected: (connected: boolean) => void,
  setRemoteTracks: (updater :(remoteTracks: MediaStreamTrack[]) => MediaStreamTrack[]) => void,
}

function JoinMenu(props: JoinMenuProps) {
  const [ip, setIp] = useState<string>("");
  const onClick = () => {
    if (props.localTrack === null) {
      console.log("Please create a valid track before joining the server");
      return;
    }
    console.log("Participant starts publishing");
    const ws = new WebSocket("ws://" + (ip !== "" ? ip : "localhost") + ":8085");
    const localTrack = props.localTrack;
    ws.onopen = () => {
      props.setConnected(true);
      startConnection(ws, localTrack, (remoteTrack: MediaStreamTrack) => {
        console.log("Participant subscription received");
        props.setRemoteTracks(prev => [...prev, remoteTrack]);
      });
    };
  };

  return <div className="ip-input-wrapper">
    <input className="ip-input" type="text" onChange={ev => setIp(ev.target.value)} 
      placeholder="Introduce the server IP"></input>
    <button className="ip-input-button" onClick={onClick}>Join</button>
  </div>;
}

export default JoinMenu;
