use rocket::http::Status;
use rocket::State;
use rocket_contrib::json::Json;
use serde::Serialize;
use std::sync::Mutex;

use crate::api::SongRequest;
use crate::api::SongRequestRsp;
use crate::music::Music;
use crate::music::yt;

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
    let file_path: Option<String> = yt::yt_dl(&song.path);
    if file_path == None {
        println!("file_path is None.");
        return SongRequestRsp::Error(Status::NotFound);
    }

    music_player.lock().unwrap().add_queue_yt(&file_path.unwrap())
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
