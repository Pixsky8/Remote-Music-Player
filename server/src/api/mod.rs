use std::sync::Mutex;

use crate::music::Music;

mod get;
mod post;

pub fn start_api(player: Mutex<Music>) -> rocket::Rocket {
    rocket::ignite()
        .manage(player)
        .mount("/", routes![get::get_queue, post::add_queue])
}
