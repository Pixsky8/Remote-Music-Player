use std::fs::File;
use std::io::BufReader;

pub struct Music {
    sink: rodio::Sink,
    path_queue: Vec<String>,

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
        }
    }

    pub fn change_volume(&mut self, volume: u8) {
        let real_value: f32 = (volume as f32) / 100.0;

        self.sink.set_volume(real_value);
    }

    // TODO: catch file errors
    pub fn add_queue(&mut self, music_path: String) {
        let file = File::open(music_path.clone()).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

        self.sink.append(source);

        self.update_queue();
        self.path_queue.insert(0, music_path.clone());
    }

    pub fn get_queue(&mut self) -> String {
        return self.path_queue.join("\n");
    }

    fn update_queue(&mut self) {
        while self.path_queue.len() > self.sink.len() {
            self.path_queue.pop();
        }
    }
}
