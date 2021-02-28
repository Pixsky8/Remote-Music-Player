import { Injectable } from "@angular/core";
import { HttpClient } from '@angular/common/http';

import { Observable, of } from 'rxjs';
import { catchError, map, tap } from 'rxjs/operators';

import { Song } from './song';

@Injectable()
export class SongService {
    queueUrl = '/api/queue'

    constructor(private http: HttpClient) { }

    getSongList(): Observable<Song[]> {
        return this.http.get<Song[]>(this.queueUrl)
            .pipe(
                catchError(this.handleError<Song[]>('SongQueue', []))
            );
    }

    private handleError<T>(operation = 'operation', result?: T) {
        return (error: any): Observable<T> => {
            console.error(error);
            return of(result as T);
        }
    }
}
