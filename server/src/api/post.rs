use rocket::State;
use rocket_contrib::json::Json;
use std::sync::Mutex;

use crate::api::SongRequest;
use crate::api::SongRequestRsp;
use crate::music::Music;

#[post("/play", data = "<song>")]
pub fn add_queue(
    music_player: State<Mutex<Music>>,
    song: Json<SongRequest>,
) -> SongRequestRsp {
    music_player.lock().unwrap().add_queue(&song.path)
}
