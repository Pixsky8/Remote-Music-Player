use rocket::State;
use std::sync::Mutex;

use crate::music::Music;

#[get("/queue")]
pub fn get_queue(music_player: State<Mutex<Music>>) -> String {
    music_player.lock().unwrap().get_queue()
}
