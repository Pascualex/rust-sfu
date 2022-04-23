import { Component, OnInit } from '@angular/core';
import { startConnection } from 'sfu-sdk-js';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit {
  title = 'sfu-web';

  public publishersCount: number = 0;
  public localTrack?: MediaStreamTrack;
  public remoteTracks: MediaStreamTrack[] = [];

  private firstPublisher = true;

  public ngOnInit(): void {
    this.getLocalTrack().then(track => this.localTrack = track);
  }

  public async newParticipant(): Promise<void> {
    if (this.localTrack == null) return;
    this.publishersCount++;
    console.log("Participant starts publishing");
    let ws = new WebSocket("ws://localhost:8085");
    await new Promise(resolve => setTimeout(resolve, 1000));
    let remoteTracks = this.remoteTracks;
    if (this.firstPublisher) {
      startConnection(ws, this.localTrack, (track: MediaStreamTrack) => {
        console.log("Participant subscription received");
        remoteTracks.push(track);
      });
      this.firstPublisher = false;
    } else {
      startConnection(ws, this.localTrack, (track: MediaStreamTrack) => {});
    }
  }

  private async getLocalTrack(): Promise<MediaStreamTrack> {
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
}
