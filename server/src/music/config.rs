use serde::Deserialize;
use std::fs::File;

#[derive(Deserialize)]
pub struct Config {
    pub min_volume: i32,
    pub max_volume: i32,
    music_path: String,
    votes_to_skip: u32,
}

impl Config {
    pub fn new() -> Config {
        Config {
            min_volume: 0,
            max_volume: 150,
            music_path: "/".to_string(),
            votes_to_skip: 1,
        }
    }

    pub fn new_from_file(config_file: &str) -> Config {
        let _file = match File::open(&config_file) {
            Ok(file) => file,
            Err(_err) => return Config::new(),
        };

        let config_str: String = match std::fs::read_to_string(&config_file) {
            Err(_) => return Config::new(),
            Ok(res_str) => res_str,
        };

        let mut res: Config = match toml::from_str(&config_str) {
            Err(_) => return Config::new(),
            Ok(res_conf) => res_conf,
        };

        res.set_music_path(&res.music_path.clone());

        return res;
    }

    pub fn set_max_volume(&mut self, new: i32) {
        self.max_volume = new;
    }

    pub fn set_min_volume(&mut self, new: i32) {
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

    pub fn nb_skip_get(&self) -> u32 {
        self.votes_to_skip
    }
}
