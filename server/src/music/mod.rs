use std::fs::File;
use std::io::BufReader;

pub mod mp3;

pub struct Config {
    min_volume: u8,
    max_volume: u8,
}

impl Config {
    pub fn new() -> Config {
        Config {
            min_volume: 0,
            max_volume: 150,
        }
    }

    pub fn set_max_volume(&mut self, new: u8) {
        self.max_volume = new;
    }

    pub fn set_min_volume(&mut self, new: u8) {
        self.min_volume = new;
    }
}

pub struct Music {
    sink: rodio::Sink,
    path_queue: Vec<mp3::Mp3>,
    config: Config,

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
            config: Config::new(),
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

    pub fn add_queue(&mut self, music_path: String) -> bool {
        let file = match File::open(music_path.clone()) {
            Ok(file) => file,
            Err(_err) => return false,
        };

        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

        self.sink.append(source);

        self.update_queue();
        self.path_queue
            .insert(0, mp3::Mp3::new(music_path.clone()).unwrap());

        return true;
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
