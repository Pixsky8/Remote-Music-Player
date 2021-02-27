import { AbsoluteSourceSpan } from '@angular/compiler';
import { Component, OnInit } from '@angular/core';

import { Song } from '../interfaces/song';

@Component({
    selector: 'app-ranking',
    templateUrl: './current.component.html',
    providers: [],
    styleUrls: ['./current.component.css']
})
export class CurrentSongComponent implements OnInit {
    currently_playing: Song | null = null;
    song_queue: Song[] = [];

    ngOnInit(): void {
        this.currently_playing = {
            name: 'Song name',
            author: 'Author',
            album: 'Album name',
            cover_path: null
        };
    }
}
