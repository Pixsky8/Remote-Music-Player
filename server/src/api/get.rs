use rocket::State;
use rocket_contrib::json::Json;
use serde::Serialize;
use std::sync::Mutex;

use crate::music::mp3::Mp3;
use crate::music::Music;

#[get("/queue")]
pub fn get_queue(music_player: State<Mutex<Music>>) -> Json<Vec<Mp3>> {
    Json(music_player.lock().unwrap().get_queue())
}

#[get("/")]
pub fn bruh_moment() -> String {
    "Bruh moment".to_string()
}

#[derive(Serialize)]
pub struct VolumeInfo {
    current_volume: i32,
    max_volume: i32,
    min_volume: i32,
}

#[get("/volume")]
pub fn get_volume_info(music_player: State<Mutex<Music>>) -> Json<VolumeInfo> {
    let min = music_player.lock().unwrap().config.min_volume;
    let max = music_player.lock().unwrap().config.max_volume;
    let curr = music_player.lock().unwrap().volume_get();

    Json(VolumeInfo {
        current_volume: curr,
        max_volume: max,
        min_volume: min,
    })
}
