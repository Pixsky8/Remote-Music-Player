use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::Deserialize;
use std::sync::Mutex;

use crate::music::mp3::Mp3;
use crate::music::Music;

mod get;
mod post;
mod volume;

#[derive(Responder)]
#[response(content_type = "json")]
pub enum SongRequestRsp {
    Body(Json<Mp3>),
    Error(Status),
}

#[derive(Deserialize)]
pub struct SongRequest {
    path: String,
}

pub fn start_api(player: Mutex<Music>) -> rocket::Rocket {
    rocket::ignite().manage(player).mount(
        "/",
        routes![
            get::get_queue,
            post::add_queue_file,
            post::add_queue_yt,
            volume::change_volume,
            get::get_volume_info,
            post::skip_vote,
            get::bruh_moment,
        ],
    )
}
