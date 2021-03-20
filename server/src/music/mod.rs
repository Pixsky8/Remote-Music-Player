use rocket::http::Status;
use rocket_contrib::json::Json;
use std::fs::File;
use std::io::BufReader;

pub mod config;
mod file;
pub mod mp3;
pub mod yt;

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
        let (stream, stream_handle) =
            rodio::OutputStream::try_default().unwrap();

        Music {
            stream: stream,
            sink: rodio::Sink::try_new(&stream_handle).unwrap(),
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

    pub fn change_volume(&mut self, volume: i32) -> i32 {
        let volume = match volume {
            v if v > self.config.max_volume => self.config.max_volume,
            v if v < self.config.min_volume => self.config.min_volume,
            v => v,
        };

        let real_value: f32 = (volume as f32) / 100.0;

        self.sink.set_volume(real_value);

        return volume;
    }

    fn replace_sink(&mut self) {
        let (stream, stream_handle) =
            rodio::OutputStream::try_default().unwrap();

        self.stream = stream;
        self.sink = rodio::Sink::try_new(&stream_handle).unwrap();
    }

    fn add_to_sink(&mut self, music_path: &str) -> Option<api::SongRequestRsp> {
        let file = match File::open(&music_path) {
            Ok(file) => file,
            Err(_err) => return Some(api::SongRequestRsp::Error(Status::NotFound)),
        };

        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

        self.sink.append(source);

        return None;
    }

    fn add_queue(
        &mut self,
        music_path: &str,
        delete_afterward: bool,
    ) -> api::SongRequestRsp {
        self.update_queue();

        let sink_rsp = self.add_to_sink(music_path);
        if !sink_rsp.is_none() {
            return sink_rsp.unwrap();
        }

        let new_mp3 =
            mp3::Mp3::from_path(&music_path, delete_afterward).unwrap();
        self.path_queue.insert(0, new_mp3.clone());

        let returned_mp3 = mp3::Mp3::new(
            file::get_filename(&new_mp3.name_get()),
            new_mp3.author_get(),
            new_mp3.album_get(),
            new_mp3.cover_get(),
        );

        return api::SongRequestRsp::Body(Json(returned_mp3));
    }

    pub fn add_queue_file(&mut self, music_path: &str) -> api::SongRequestRsp {
        if !file::allow_access(&music_path) {
            return api::SongRequestRsp::Error(Status::Forbidden);
        }

        let music_path: String =
            self.config.get_music_path().to_string() + music_path;

        self.add_queue(&music_path, false)
    }

    pub fn add_queue_yt(&mut self, file_path: &str) -> api::SongRequestRsp {
        self.add_queue(file_path, true)
    }

    pub fn get_queue(&mut self) -> Vec<mp3::Mp3> {
        self.update_queue();

        let mut queue = vec![];

        for elem in &self.path_queue {
            queue.push(mp3::Mp3::new(
                file::get_filename(&elem.name_get()),
                elem.author_get(),
                elem.album_get(),
                elem.cover_get(),
            ));
        }

        return queue;
    }

    fn update_queue(&mut self) {
        while self.path_queue.len() > self.sink.len() {
            let mp3_file_opt: Option<mp3::Mp3> = self.path_queue.pop();
            if mp3_file_opt.is_none() {
                continue;
            }

            let mp3_file = mp3_file_opt.unwrap();
            if mp3_file.is_temporary() {
                file::delete_tmp_file(&mp3_file.path_get());
            }
        }
    }

    pub fn skip_vote(&mut self) -> Option<u32> {
        self.update_queue();

        if self.path_queue.is_empty() {
            return None;
        }

        let last_value_index = self.path_queue.len() - 1;
        let nb_votes = self.path_queue[last_value_index].increment_skip();

        println!("Votes: {}", nb_votes);

        if nb_votes >= self.config.nb_skip_get() {
            println!("Skipping");
            self.sink.stop();
            self.replace_sink();
            self.path_queue.pop();

            if last_value_index == 0 {
                return None;
            }

            let mut i: usize = last_value_index - 1;
            while i > 0 {
                self.add_to_sink(&self.path_queue[i].path_get());
                i -= 1;
            }
            self.add_to_sink(&self.path_queue[0].path_get());

            Some(0);
        }

        Some(nb_votes)
    }
}
