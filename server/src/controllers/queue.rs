use log::{info, warn};
use rocket::http::Status;
use rocket::State;
use rocket_contrib::json::Json;
use std::sync::Mutex;

use crate::music::mp3::Mp3;
use crate::music::Music;
use crate::music::yt;
use crate::requests::song::SongRequest;
use crate::responses::{song::SongRequestRsp, vote::SkipVotes};

#[get("/queue")]
pub fn get_queue(music_player: State<Mutex<Music>>) -> Json<Vec<Mp3>> {
    Json(music_player.lock().unwrap().get_queue())
}

#[post("/play", data = "<song>")]
pub fn add_queue_file(
    music_player: State<Mutex<Music>>,
    song: Json<SongRequest>,
) -> SongRequestRsp {
    info!("New Song Request: {}", &song.path);
    music_player.lock().unwrap().add_queue_file(&song.path)
}

#[post("/ytplay", data = "<song>")]
pub fn add_queue_yt(
    music_player: State<Mutex<Music>>,
    song: Json<SongRequest>,
) -> SongRequestRsp {
    info!("Fetching from youtube: {}", &song.path);

    let file_path_opt: Option<String> = yt::yt_dl(&song.path);
    if file_path_opt == None {
        warn!("youtube-dl error");
        return SongRequestRsp::Error(Status::NotFound);
    }

    let file_path: String = file_path_opt.unwrap();
    info!("Enqueued (youtube): {}", file_path);

    music_player.lock().unwrap().add_queue_yt(&file_path)
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
