#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use log;
use log::error;
use std::sync::Mutex;
use termion::color;

mod controllers;
mod music;
mod requests;
mod responses;

fn main() {
    let config_path: Option<String> = std::env::args().nth(1);

    let player = match config_path {
        None => Mutex::new(music::Music::new()),
        Some(v) => Mutex::new(music::Music::new_from_file(&v)),
    };

    println!(
        "{}{}",
        color::Fg(color::Cyan),
        player.lock().unwrap().config.to_string()
    );

    let web_server = controllers::start_api(player);
    web_server.launch();
}
