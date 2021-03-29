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
            music_path: "./music".to_string(),
            votes_to_skip: 1,
        }
    }

    pub fn new_from_file(config_file: &str) -> Config {
        let _file = match File::open(&config_file) {
            Ok(file) => file,
            Err(_err) => {
                println!("{} does not exist, using default config", config_file);
                return Config::new();
            },
        };

        let config_str: String = match std::fs::read_to_string(&config_file) {
            Err(_) => {
                println!("Could not read {}, using default config", config_file);
                return Config::new();
            },
            Ok(res_str) => res_str,
        };

        let mut res: Config = match toml::from_str(&config_str) {
            Err(_) => {
                println!("Could not parse {}, using default config", config_file);
                return Config::new();
            },
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

    pub fn to_string(&self) -> String {
        let mut res: String = String::new();

        res.push_str("Music Path: ");
        res.push_str(&self.music_path);

        res.push_str("\nNumber of votes to skip: ");
        res.push_str(&self.votes_to_skip.to_string());

        res.push_str("\nMaximum Volume: ");
        res.push_str(&self.max_volume.to_string());

        res.push_str("\nMinimum Volume: ");
        res.push_str(&self.min_volume.to_string());

        return res;
    }
}
