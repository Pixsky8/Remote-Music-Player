import { Component, OnInit } from '@angular/core';
import { MatSnackBar } from '@angular/material/snack-bar';

import { Song } from '../interfaces/get/song';
import { SongService } from '../interfaces/get/song.service';

import { Skip } from '../interfaces/post/skip';
import { SkipService } from '../interfaces/post/skip.service';

@Component({
    selector: 'app-song-list',
    templateUrl: './song-list.component.html',
    providers: [SongService, SkipService],
    styleUrls: ['./song-list.component.css']
})
export class SongListComponent implements OnInit {
    currently_playing: Song | null = null;
    song_queue: Song[] = [];

    constructor(private songService: SongService,
                private skipService: SkipService,
                private snackBar: MatSnackBar) { }

    ngOnInit(): void {
        this.updateQueue();
    }

    updateQueue(): void {
        this.songService.getSongList()
            .subscribe(queue => (this.changeQueueValue(queue)));
    }

    private changeQueueValue(new_queue: Song[]) {
        var curr: Song | undefined = new_queue.pop();
        if (curr == undefined)
            this.currently_playing == null
        else
            this.currently_playing = curr;

        this.song_queue = new_queue.reverse();
    }

    skipVote(): void {
        this.skipService.postSkip()
            .subscribe(skip_votes => {
                if (typeof skip_votes == "number") {
                    this.displaySnackBar("Cannot vote to skip at the moment.");
                    return;
                }

                this.displaySnackBar("Total number of votes: " + skip_votes.votes);
            });
    }

    private displaySnackBar(message: string): void {
        this.snackBar.open(message, "", {
            duration: 5000,
        });
    }
}
