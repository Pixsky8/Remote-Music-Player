use rocket::State;
use rocket_contrib::json::Json;
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
