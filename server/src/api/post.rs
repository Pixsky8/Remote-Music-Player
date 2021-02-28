use rocket::http::Status;
use rocket::State;
use rocket_contrib::json::Json;
use serde::Deserialize;
use std::sync::Mutex;

use crate::music::Music;

#[derive(Deserialize)]
pub struct SongRequest {
    path: String,
}

#[post("/play", data = "<song>")]
pub fn add_queue(
    music_player: State<Mutex<Music>>,
    song: Json<SongRequest>,
) -> Status {
    if !music_player.lock().unwrap().add_queue(song.path.clone()) {
        return Status::NotFound;
    }

    Status::Ok
}
