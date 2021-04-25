import { Component, OnInit } from '@angular/core';
import { MatSnackBar } from '@angular/material/snack-bar';

import { VolumeRequest, VolumeResponse } from '../interfaces/volume/volume';
import { VolumeService } from '../interfaces/volume/volume.service'

@Component({
    selector: 'app-volume-changer',
    templateUrl: './volume.component.html',
    providers: [VolumeService],
    styleUrls: ['./volume.component.css']
})
export class VolumeComponent implements OnInit {
    min_volume: number = 0;
    max_volume: number = 0;
    curr_volume: number = 0;

    constructor(private volumeService: VolumeService,
                private snackBar: MatSnackBar) { }

    ngOnInit(): void {
        this.getVolumeInfo();
    }

    getVolumeInfo(): void {
        this.volumeService.getVolumeInfo()
            .subscribe(rsp => {
                this.min_volume = rsp.min_volume;
                this.max_volume = rsp.max_volume;
                this.curr_volume = rsp.current_volume;
            });
    }

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

        var volumeRqst: VolumeRequest = {
            new_volume: Number(vol),
        };

        this.postRequestVolume(volumeRqst);
    }

    postRequestVolume(put: VolumeRequest): void {
        this.volumeService.requestVolume(put)
            .subscribe(putRsp => {
                this.min_volume = putRsp.min_volume;
                this.max_volume = putRsp.max_volume;
                this.curr_volume = putRsp.current_volume;
                this.snackBar.open("New Volume set: " + putRsp.current_volume, "", {
                    duration: 5000,
                });
            });
    }
}
