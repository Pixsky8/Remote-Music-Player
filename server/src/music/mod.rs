use rocket::http::Status;
use std::fs::File;
use std::io::BufReader;

pub mod config;
pub mod mp3;

mod file;

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

    pub fn add_queue(&mut self, music_path: &str) -> Status {
        if !file::allow_access(&music_path) {
            return Status::Forbidden;
        }

        let music_path: String =
            self.config.get_music_path().to_string() + music_path;

        println!("{}", music_path);

        let file = match File::open(&music_path) {
            Ok(file) => file,
            Err(_err) => return Status::NotFound,
        };

        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

        self.sink.append(source);

        self.update_queue();
        self.path_queue
            .insert(0, mp3::Mp3::new(&music_path).unwrap());

        return Status::Ok;
    }

    pub fn get_queue(&mut self) -> Vec<mp3::Mp3> {
        self.path_queue.clone()
    }

    fn update_queue(&mut self) {
        while self.path_queue.len() > self.sink.len() {
            self.path_queue.pop();
        }
    }
}
