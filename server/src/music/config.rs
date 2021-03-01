pub struct Config {
    pub min_volume: u8,
    pub max_volume: u8,
    music_path: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            min_volume: 0,
            max_volume: 150,
            music_path: "/".to_string(),
        }
    }

    pub fn set_max_volume(&mut self, new: u8) {
        self.max_volume = new;
    }

    pub fn set_min_volume(&mut self, new: u8) {
        self.min_volume = new;
    }

    pub fn set_music_path(&mut self, new: &str) {
        self.music_path = new.to_string();
        if !self.music_path.ends_with("/") {
            self.music_path += "/";
        }
    }

    pub fn get_music_path(&mut self) -> &str {
        &self.music_path
    }
}
