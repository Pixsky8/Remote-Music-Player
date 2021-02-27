use std::fs::File;
use std::io::BufReader;

pub fn add_queue(sink: &rodio::Sink, music_path: String) {
    let file = File::open(music_path).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    sink.append(source);
}
