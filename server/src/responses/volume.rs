use serde::Serialize;

#[derive(Serialize)]
pub struct VolumeInfo {
    pub current_volume: i32,
    pub max_volume: i32,
    pub min_volume: i32,
}
