import { Injectable } from "@angular/core";
import { HttpClient, HttpHeaders } from '@angular/common/http';

import { Observable, of } from 'rxjs';
import { catchError, tap } from 'rxjs/operators';

import { Skip } from './skip';

@Injectable()
export class SkipService {
    songSkipUrl = '/api/skip'

    constructor(private http: HttpClient) { }

    httpOptions = {
        headers: new HttpHeaders({ 'Content-Type': 'application/json' }),
    };

    postSkip(): Observable<Skip | number> {
        return this.http.post<Skip>(this.songSkipUrl,
            this.httpOptions
        ).pipe(
            tap((result: Skip) =>
                console.log('voted to skip song: ' + result.votes)),
            catchError(this.handleError<Skip>('PostSkip'))
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
