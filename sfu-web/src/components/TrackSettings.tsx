import { useEffect, useState } from "react";
import Participant from "./Participant";
import "./TrackSettings.css";

interface TrackSettingsProps {
  setTrack: (track: MediaStreamTrack | null) => void,
}

function TrackSettings(props: TrackSettingsProps) {
  const devices = useGetDevices();
  const [device, setDevice] = useState<MediaDeviceInfo | null>(null);
  const [audio, setAudio] = useState<boolean>(true);
  const localTrack = useDeviceToTrack(device, audio);
  const [showDropdown, setShowDropdown] = useState<boolean>(false); 

  useEffect(() => props.setTrack(localTrack), [localTrack]);

  let labelRows = [];
  if (showDropdown) {
    const clickAndClose = (device: MediaDeviceInfo | null) => {
      setDevice(device);
      setShowDropdown(false);
    };
    labelRows.push(
      <button className="device" key="" onClick={() => clickAndClose(null)}>
        None
      </button>
    );
    for (const currentDevice of devices) {
      if (currentDevice.kind !== "videoinput") continue;
      if (currentDevice.label === "" || currentDevice.deviceId === "") continue;
      const classes = "device" + (currentDevice === device ? " selected" : "");
      const onClick = () => clickAndClose(currentDevice);
      labelRows.push(
        <button className={classes} key={currentDevice.deviceId} onClick={onClick}>
          {currentDevice.label}
        </button>
      );
    }
  }

  return <div className="card">
    <h5 className="title">Track settings</h5>
    <div className="audio-row">
      <p>Enable audio</p>
      <input type="checkbox" defaultChecked onChange={ev => setAudio(ev.target.checked)}></input>
    </div>
    <div className="participant-wrapper">
      <Participant track={localTrack} mirrored={true} />
    </div>
    <div className="dropdown-wrapper">
      <button className="dropdown-button" onClick={() => setShowDropdown(!showDropdown)}>
        {device != null ? device.label : "Select device"}
      </button>
      {showDropdown ? <div className="dropdown">{labelRows}</div> : null}
    </div>
  </div>;
}

function useGetDevices(): MediaDeviceInfo[] {
  const [devicesInfo, setDevicesInfo] = useState<MediaDeviceInfo[]>([]);
  useEffect(() => {
    navigator.mediaDevices.getUserMedia({video: true, audio: true}).then(() => {
      navigator.mediaDevices.enumerateDevices().then(di => setDevicesInfo(di));
    });
  }, []);
  return devicesInfo;
}

function useDeviceToTrack(device: MediaDeviceInfo | null, audio: boolean): MediaStreamTrack | null {
  const [track, setTrack] = useState<MediaStreamTrack | null>(null);
  useEffect(() => {
    if (device === null) {
      setTrack(null);
      return;
    }
    navigator.mediaDevices.getUserMedia({
      video: {
        width: 1280,
        height: 720,
        frameRate: 24,
        deviceId: device.deviceId,
      },
      audio,
    }).then(stream => {
      if (stream.getVideoTracks().length > 0) {
        setTrack(stream.getVideoTracks()[0]);
      }
    });
  }, [device, audio]);
  return track;
}

export default TrackSettings;
