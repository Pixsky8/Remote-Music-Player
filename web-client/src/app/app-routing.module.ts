import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';

import { SongListComponent } from './song-list/song-list.component'
import { RequestSongComponent } from './request-song/request-song.component'

const routes: Routes = [
    { path: '', component: SongListComponent, pathMatch: 'full' },
    { path: 'request', component: RequestSongComponent }
];

@NgModule({
    imports: [RouterModule.forRoot(routes)],
    exports: [RouterModule]
})
export class AppRoutingModule { }
