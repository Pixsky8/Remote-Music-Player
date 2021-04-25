export interface VolumeRequest {
    new_volume: number,
};

export interface VolumeResponse {
    current_volume: number,
    max_volume: number,
    min_volume: number,
};
