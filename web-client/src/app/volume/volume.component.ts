import { Component, OnInit } from '@angular/core';
import { MatSnackBar } from '@angular/material/snack-bar';

import { Volume } from '../interfaces/volume/volume';
import { VolumeService } from '../interfaces/volume/volume.service'

@Component({
    selector: 'app-volume-changer',
    templateUrl: './volume.component.html',
    providers: [VolumeService],
    styleUrls: ['./volume.component.css']
})
export class VolumeComponent implements OnInit {
    constructor(private volumeService: VolumeService,
                private snackBar: MatSnackBar) { }

    ngOnInit(): void { }

    isValidNumber(num: string): boolean {
        var len = num.length;
        for (var i = 0; i < len; i++)
        {
            if (num[i] < '0' || '9' < num[i])
                return false;
        }
        return true;
    }

    requestVolume(vol: string): void {
        if (!this.isValidNumber(vol))
        {
            this.snackBar.open("Volume entered is not valid.", "", {
                duration: 5000,
            });
            return;
        }

        var volumeRqst: Volume = {
            new_volume: Number(vol),
        };

        this.postRequestVolume(volumeRqst);
    }

    postRequestVolume(put: Volume): void {
        this.volumeService.requestVolume(put)
            .subscribe(putRsp => {
                this.snackBar.open("New Volume set: " + putRsp.new_volume, "", {
                    duration: 5000,
                });
            });
    }
}
