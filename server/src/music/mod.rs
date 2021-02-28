use std::fs::File;
use std::io::BufReader;

pub struct Music {
    sink: rodio::Sink,

    #[allow(dead_code)]
    stream: rodio::OutputStream,
}

impl Music {
    pub fn new() -> Music {
        let (stream_, stream_handle_) =
            rodio::OutputStream::try_default().unwrap();

        Music {
            stream: stream_,
            sink: rodio::Sink::try_new(&stream_handle_).unwrap(),
        }
    }

    pub fn add_queue(&mut self, music_path: String) {
        let file = File::open(music_path).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

        self.sink.append(source);
    }

    pub fn change_volume(&mut self, volume: u8) {
        let real_value: f32 = (volume as f32) / 100.0;

        self.sink.set_volume(real_value);
    }
}
