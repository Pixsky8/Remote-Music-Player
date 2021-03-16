use id3::Tag;
use serde::Serialize;
use std::error::Error;
use std::option::Option;

#[derive(Serialize, Clone)]
pub struct Mp3 {
    name: String,
    author: Option<String>,
    album: Option<String>,
    album_cover: Option<String>,
    is_path: bool,
    delete_afterward: bool,
}

impl Mp3 {
    pub fn new(path: &str, delete_afterward: bool) -> Result<Mp3, Box<dyn Error>> {
        let tag = Tag::read_from_path(path)?;

        let mut mp3_file = Mp3 {
            name: "".to_string(),
            author: None,
            album: None,
            album_cover: None,
            is_path: false,
            delete_afterward: delete_afterward,
        };

        if let Some(name) = tag.title() {
            mp3_file.name = name.to_string();
        }
        else {
            mp3_file.name = path.to_string();
            mp3_file.is_path = true;
        }

        if let Some(author) = tag.artist() {
            mp3_file.author = Some(author.to_string());
        }

        if let Some(album) = tag.album() {
            mp3_file.album = Some(album.to_string());
        }

        return Ok(mp3_file);
    }

    pub fn is_temporary(&self) -> bool {
        self.delete_afterward
    }

    pub fn use_path(&self) -> bool {
        self.is_path
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
