import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { HttpClientModule } from '@angular/common/http';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';

// Components
import { SongListComponent } from './song-list/song-list.component';
import { RequestSongComponent } from './request-song/request-song.component';

@NgModule({
    declarations: [
        AppComponent,
        SongListComponent,
        RequestSongComponent
    ],
    imports: [
        BrowserModule,
        AppRoutingModule,
        HttpClientModule
    ],
    providers: [],
    bootstrap: [AppComponent]
})
export class AppModule { }
