use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::music::mp3::Mp3;

#[derive(Responder)]
#[response(content_type = "json")]
pub enum SongRequestRsp {
    Body(Json<Mp3>),
    Error(Status),
}
