#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use std::sync::{Arc, Mutex};

mod api;
mod music;

fn main() {
    let mut player = Mutex::new(music::Music::new());

    let web_server = api::start_api(player);
    web_server.launch();
}
