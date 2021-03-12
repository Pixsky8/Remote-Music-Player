use rocket::http::Status;
use rocket_contrib::json::Json;
use std::fs::File;
use std::io::BufReader;

pub mod config;
mod file;
pub mod mp3;
mod yt;

use crate::api;

pub struct Music {
    sink: rodio::Sink,
    path_queue: Vec<mp3::Mp3>,
    pub config: config::Config,

    #[allow(dead_code)]
    stream: rodio::OutputStream,
}

unsafe impl Send for Music {}
unsafe impl Sync for Music {}

impl Music {
    pub fn new() -> Music {
        let (stream_, stream_handle_) =
            rodio::OutputStream::try_default().unwrap();

        Music {
            stream: stream_,
            sink: rodio::Sink::try_new(&stream_handle_).unwrap(),
            path_queue: Vec::new(),
            config: config::Config::new(),
        }
    }

    pub fn new_from_file(config_file: &str) -> Music {
        let (stream_, stream_handle_) =
            rodio::OutputStream::try_default().unwrap();

        Music {
            stream: stream_,
            sink: rodio::Sink::try_new(&stream_handle_).unwrap(),
            path_queue: Vec::new(),
            config: config::Config::new_from_file(config_file),
        }
    }

    pub fn change_volume(&mut self, volume: u8) {
        let volume = match volume {
            v if v > self.config.max_volume => self.config.max_volume,
            v if v < self.config.min_volume => self.config.min_volume,
            v => v,
        };

        let real_value: f32 = (volume as f32) / 100.0;

        self.sink.set_volume(real_value);
    }

    fn add_queue(&mut self, music_path: &str) -> api::SongRequestRsp {
        self.update_queue();

        let file = match File::open(&music_path) {
            Ok(file) => file,
            Err(_err) => return api::SongRequestRsp::Error(Status::NotFound),
        };

        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

        self.sink.append(source);

        let new_mp3 = mp3::Mp3::new(&music_path).unwrap();
        self.path_queue.insert(0, new_mp3.clone());

        return api::SongRequestRsp::Body(Json(new_mp3));
    }

    pub fn add_queue_file(&mut self, music_path: &str) -> api::SongRequestRsp {
        if !file::allow_access(&music_path) {
            return api::SongRequestRsp::Error(Status::Forbidden);
        }

        let music_path: String =
            self.config.get_music_path().to_string() + music_path;

        self.add_queue(&music_path)
    }

    pub fn add_queue_yt(&mut self, music_url: &str) -> api::SongRequestRsp {
        let file_path: Option<String> = yt::yt_dl(music_url);
        if file_path == None {
            return api::SongRequestRsp::Error(Status::NotFound);
        }

        self.add_queue(&file_path.unwrap())
    }

    pub fn get_queue(&mut self) -> Vec<mp3::Mp3> {
        self.update_queue();

        self.path_queue.clone()
    }

    fn update_queue(&mut self) {
        while self.path_queue.len() > self.sink.len() {
            self.path_queue.pop();
        }
    }
}
