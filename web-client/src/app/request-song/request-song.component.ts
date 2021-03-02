import { Component, OnInit } from '@angular/core';

import { RequestService } from '../interfaces/post/request.service';

import { Request } from '../interfaces/post/request'
import { Song } from '../interfaces/get/song'

@Component({
    selector: 'app-request-song',
    templateUrl: './request-song.component.html',
    providers: [RequestService],
    styleUrls: ['./request-song.component.css']
})
export class RequestSongComponent implements OnInit {
    added_song: Song | null = null;

    constructor(private requestService: RequestService) { }

    ngOnInit(): void { }

    requestSong(file: string): void {
        if (file == "") {
            window.alert("Enter a song to send the request.");
            return;
        }

        var post: Request = {
            path: file,
        };

        this.postSongRequest(post);
    }

    private postSongRequest(post: Request): void {
        this.requestService.postRace(post)
            .subscribe(postRsp => {
                this.added_song = postRsp
            });
    }
}
