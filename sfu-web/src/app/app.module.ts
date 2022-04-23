import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppComponent } from './app.component';
import { ParticipantComponent } from './participant/participant.component';
import { ParticipantGridComponent } from './participant-grid/participant-grid.component';

@NgModule({
  declarations: [
    AppComponent,
    ParticipantComponent,
    ParticipantGridComponent
  ],
  imports: [
    BrowserModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
