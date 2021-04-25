use std::sync::Mutex;

use crate::music::Music;

mod get;
mod post;
mod volume;

pub fn start_api(player: Mutex<Music>) -> rocket::Rocket {
    rocket::ignite().manage(player).mount(
        "/",
        routes![
            // Queue
            get::get_queue,
            post::add_queue_file,
            post::add_queue_yt,
            // Volume
            volume::get_volume_info,
            volume::change_volume,
            // Skip
            post::skip_vote,
            // Misc
            get::bruh_moment,
        ],
    )
}
