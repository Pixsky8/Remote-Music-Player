use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use crate::music::Music;

#[derive(Serialize, Deserialize)]
pub struct Volume {
    new_volume: i32,
}

#[put("/volume", data = "<volume>")]
pub fn change_volume(
    music_player: State<Mutex<Music>>,
    volume: Json<Volume>,
) -> Json<Volume> {
    let rsp: Volume = {
        Volume {
            new_volume: music_player
                .lock()
                .unwrap()
                .change_volume(volume.new_volume),
        }
    };
    return Json(rsp);
}
