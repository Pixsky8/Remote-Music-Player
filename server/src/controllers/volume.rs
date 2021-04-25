use rocket::State;
use rocket_contrib::json::Json;
use std::sync::Mutex;

use crate::music::Music;
use crate::requests::volume::VolumeRequest;
use crate::responses::volume::VolumeInfo;

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

#[put("/volume", data = "<volume>")]
pub fn change_volume(
    music_player: State<Mutex<Music>>,
    volume: Json<VolumeRequest>,
) -> Json<VolumeInfo> {
    let curr = music_player
        .lock()
        .unwrap()
        .change_volume(volume.new_volume);

    let min = music_player.lock().unwrap().config.min_volume;
    let max = music_player.lock().unwrap().config.max_volume;

    return Json(VolumeInfo {
        current_volume: curr,
        max_volume: max,
        min_volume: min,
    });
}
