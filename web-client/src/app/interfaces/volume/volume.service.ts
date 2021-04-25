import { Injectable } from "@angular/core";
import { HttpClient, HttpHeaders } from '@angular/common/http';

import { Observable, of } from 'rxjs';
import { catchError, tap } from 'rxjs/operators';

import { VolumeRequest, VolumeResponse } from './volume';

@Injectable()
export class VolumeService {
    volumeRequestUrl = '/api/volume'

    constructor(private http: HttpClient) { }

    httpOptions = {
        headers: new HttpHeaders({ 'Content-Type': 'application/json' })
    };

    getVolumeInfo(): Observable<VolumeResponse> {
        return this.http.get<VolumeResponse>(this.volumeRequestUrl,
            this.httpOptions
        ).pipe(
            catchError(this.handleError<VolumeResponse>('VolumeInfo'))
        );
    }

    requestVolume(vol: VolumeRequest): Observable<VolumeResponse> {
        const volJson = JSON.stringify(vol);
        return this.http.put<VolumeResponse>(this.volumeRequestUrl,
            volJson,
            this.httpOptions
        ).pipe(
            tap((result: VolumeResponse) =>
                console.log(result)),
            catchError(this.handleError<VolumeResponse>('VolumeRequest'))
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
