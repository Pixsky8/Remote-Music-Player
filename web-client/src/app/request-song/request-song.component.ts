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
    post: Request | null = null;

    constructor(private requestService: RequestService) { }

    ngOnInit(): void {
        this.post = {
            path: 'Heartbeat.mp3'
        };
        this.postSongRequest();
    }

    private postSongRequest(): void {
        if (this.post == null) {
            window.alert("Enter a song to send the request.");
            return;
        }

        this.requestService.postRace(this.post)
            .subscribe(postRsp => {
                console.log(postRsp)
            });
    }
}
