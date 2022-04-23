import { Component, ElementRef, Input, OnInit, ViewChild } from '@angular/core';

@Component({
  selector: 'app-participant',
  templateUrl: './participant.component.html',
  styleUrls: ['./participant.component.scss']
})
export class ParticipantComponent implements OnInit {
  @ViewChild('player', { static: true })
  public player?: ElementRef<HTMLVideoElement>;

  @Input()
  public track?: MediaStreamTrack;

  ngOnInit(): void {
    if (this.track == null) return;
    if (this.player == null) return;
    this.player.nativeElement.srcObject = new MediaStream([this.track]);
  }
}
