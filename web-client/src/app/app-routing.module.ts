import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';

import { CurrentSongComponent } from './current/current.component'

const routes: Routes = [
    { path: '', component: CurrentSongComponent, pathMatch: 'full'}
];

@NgModule({
    imports: [RouterModule.forRoot(routes)],
    exports: [RouterModule]
})
export class AppRoutingModule { }
