mod music;

fn main() {
    let mut player = music::Music::new();

    player.add_queue(String::from("/mnt/hdd/Musique/Heartbeat.mp3"));

    player.change_volume(140);

    loop {}
}
