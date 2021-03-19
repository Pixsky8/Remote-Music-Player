use rocket::State;
use rocket_contrib::json::Json;
use serde::Serialize;
use std::sync::Mutex;

use crate::api::SongRequest;
use crate::api::SongRequestRsp;
use crate::music::Music;

#[derive(Serialize)]
pub struct SkipVotes {
    votes: u32,
}

#[post("/play", data = "<song>")]
pub fn add_queue_file(
    music_player: State<Mutex<Music>>,
    song: Json<SongRequest>,
) -> SongRequestRsp {
    music_player.lock().unwrap().add_queue_file(&song.path)
}

#[post("/ytplay", data = "<song>")]
pub fn add_queue_yt(
    music_player: State<Mutex<Music>>,
    song: Json<SongRequest>,
) -> SongRequestRsp {
    music_player.lock().unwrap().add_queue_yt(&song.path)
}

#[post("/skip")]
pub fn skip_vote(music_player: State<Mutex<Music>>) -> Option<Json<SkipVotes>> {
    let votes = music_player.lock().unwrap().skip_vote();

    if votes.is_none() {
        return None;
    }

    Some(Json(SkipVotes {
        votes: votes.unwrap(),
    }))
}
