use serde::Deserialize;

#[derive(Deserialize)]
pub struct VolumeRequest {
    pub new_volume: i32,
}
