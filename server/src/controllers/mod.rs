use std::sync::Mutex;

use crate::music::Music;

mod misc;
mod queue;
mod volume;

pub fn start_api(player: Mutex<Music>) -> rocket::Rocket {
    rocket::ignite().manage(player).mount(
        "/",
        routes![
            // Queue
            queue::get_queue,
            queue::add_queue_file,
            gueue::add_queue_yt,
            queue::skip_vote,
            // Volume
            volume::get_volume_info,
            volume::change_volume,
            // Misc
            misc::bruh_moment,
        ],
    )
}
