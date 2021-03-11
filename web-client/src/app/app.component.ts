import { Component } from '@angular/core';
import { MatToolbarModule } from '@angular/material/toolbar';
import { Router, Routes } from '@angular/router';

@Component({
    selector: 'app-root',
    templateUrl: './app.component.html',
    styleUrls: ['./app.component.css']
})
export class AppComponent {
    title = 'Music Player';
    navTabs = [
        { path: '/', label: 'Queue' },
        { path: '/request', label: 'Request' },
    ];
}
