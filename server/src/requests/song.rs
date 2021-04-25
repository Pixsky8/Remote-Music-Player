use serde::Deserialize;

#[derive(Deserialize)]
pub struct SongRequest {
    pub path: String,
}
