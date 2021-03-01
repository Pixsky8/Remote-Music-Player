import { Component, OnInit } from '@angular/core';

import { Song } from '../interfaces/get/song';
import { SongService } from '../interfaces/get/song.service'

@Component({
    selector: 'app-song-list',
    templateUrl: './song-list.component.html',
    providers: [SongService],
    styleUrls: ['./song-list.component.css']
})
export class SongListComponent implements OnInit {
    currently_playing: Song | null = null;
    song_queue: Song[] = [];

    constructor(private songService: SongService) { }

    ngOnInit(): void {
        this.updateQueue();
    }

    updateQueue(): void {
        this.songService.getSongList()
            .subscribe(queue => (this.changeQueueValue(queue)));
    }

    changeQueueValue(new_queue: Song[]) {
        var curr: Song | undefined = new_queue.pop();
        if (curr == undefined)
            this.currently_playing == null
        else
            this.currently_playing = curr;

        this.song_queue = new_queue.reverse();
    }
}
