import { Injectable } from "@angular/core";
import { HttpClient, HttpHeaders } from '@angular/common/http';

import { Observable, of } from 'rxjs';
import { catchError, tap } from 'rxjs/operators';

import { Request } from './request';
import { Song } from '../get/song';

@Injectable()
export class RequestService {
    songRequestUrl = '/api/play'
    songRequestYtUrl = '/api/ytplay'

    constructor(private http: HttpClient) { }

    httpOptions = {
        headers: new HttpHeaders({ 'Content-Type': 'application/json' })
    };

    requestSong(requestPost: Request): Observable<Song | number> {
        const songPostJson = JSON.stringify(requestPost);
        return this.http.post<Song>(this.songRequestUrl,
            songPostJson,
            this.httpOptions
        ).pipe(
            tap((result: Song) =>
                console.log('enqueued new song: ' + result.name)),
            catchError(this.handleError<Song>('PostSongRequest'))
        );
    }

    requestYtSong(requestPost: Request): Observable<Song | number> {
        const songPostJson = JSON.stringify(requestPost);
        return this.http.post<Song>(this.songRequestYtUrl,
            songPostJson,
            this.httpOptions
        ).pipe(
            tap((result: Song) =>
                console.log('enqueued new yt song: ' + result.name)),
            catchError(this.handleError<Song>('PostSongRequest'))
        );
    }

    private handleError<T>(operation = 'operation', result?: T) {
        return (error: any): Observable<T | number> => {
            if (error.error instanceof ErrorEvent) {
                console.error(error);
                return of(result as T);
            }

            return of(error.status);
        }
    }
}
