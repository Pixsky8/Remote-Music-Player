import { Injectable } from "@angular/core";
import { HttpClient, HttpHeaders } from '@angular/common/http';

import { Observable, of } from 'rxjs';
import { catchError, tap } from 'rxjs/operators';

import { Volume } from './volume';

@Injectable()
export class VolumeService {
    volumeRequestUrl = '/api/volume'

    constructor(private http: HttpClient) { }

    httpOptions = {
        headers: new HttpHeaders({ 'Content-Type': 'application/json' })
    };

    requestVolume(vol: Volume): Observable<Volume> {
        const volJson = JSON.stringify(vol);
        return this.http.put<Volume>(this.volumeRequestUrl,
            volJson,
            this.httpOptions
        ).pipe(
            tap((result: Volume) =>
                console.log('new volume: ' + result.new_volume)),
            catchError(this.handleError<Volume>('VolumeRequest'))
        );
    }

    private handleError<T>(operation = 'operation', result?: T) {
        return (error: any): Observable<T> => {
            if (error.error instanceof ErrorEvent) {
                console.error(error);
                return of(result as T);
            }

            return of(error.status);
        }
    }
}
