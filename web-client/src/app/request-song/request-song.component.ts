import { Component, OnInit } from '@angular/core';
import { MatSnackBar } from '@angular/material/snack-bar';

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
    constructor(private requestService: RequestService,
                private snackBar: MatSnackBar) { }

    ngOnInit(): void { }

    requestSong(file: string): void {
        if (file == "") {
            this.snackBar.open("Enter a song to send the request.", "", {
                duration: 5000,
            });
            return;
        }

        var post: Request = {
            path: file,
        };

        this.postSongRequest(post);
    }

    requestYtSong(file: string): void {
        if (file == "") {
            this.snackBar.open("Enter a song to send the request.", "", {
                duration: 5000,
            });
            return;
        }

        var post: Request = {
            path: file,
        };

        this.postYtSongRequest(post);
    }

    private postSongRequest(post: Request): void {
        this.requestService.requestSong(post)
            .subscribe(postRsp => {
                if (typeof postRsp == "number") {
                    this.snackBar.open("Song was not found.", "", {
                        duration: 5000,
                    });
                    return;
                }

                this.displayRequestResponse(postRsp);
            });
    }

    private postYtSongRequest(post: Request): void {
        this.requestService.requestYtSong(post)
            .subscribe(postRsp => {
                if (typeof postRsp == "number") {
                    this.snackBar.open("Song was not found.", "", {
                        duration: 5000,
                    });
                    return;
                }

                this.displayRequestResponse(postRsp);
            });
    }

    private displayRequestResponse(resp: Song): void {
        this.snackBar.open("Enqueued song: " + resp.name, "", {
            duration: 5000,
        });
    }
}
