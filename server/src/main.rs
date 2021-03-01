#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use std::sync::Mutex;

mod api;
mod music;

fn main() {
    let player = Mutex::new(music::Music::new_from_file("Music.toml"));

    let web_server = api::start_api(player);
    web_server.launch();
}
