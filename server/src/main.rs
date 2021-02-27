mod music;

fn main() {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();

    music::play::add_queue(&sink,
                           String::from("/mnt/hdd/Musique/Heartbeat.mp3"));

    loop {}
}
