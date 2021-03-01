use id3::Tag;
use serde::Serialize;
use std::error::Error;
use std::option::Option;

#[derive(Serialize, Clone)]
pub struct Mp3 {
    name: Option<String>,
    author: Option<String>,
    album: Option<String>,
    album_cover: Option<String>,
}

impl Mp3 {
    pub fn new(path: &str) -> Result<Mp3, Box<dyn Error>> {
        let tag = Tag::read_from_path(path)?;

        let mut mp3_file = Mp3 {
            name: None,
            author: None,
            album: None,
            album_cover: None,
        };

        if let Some(name) = tag.title() {
            mp3_file.name = Some(name.to_string());
        }
        else {
            mp3_file.name = Some(path.to_string());
        }

        if let Some(author) = tag.artist() {
            mp3_file.author = Some(author.to_string());
        }

        if let Some(album) = tag.album() {
            mp3_file.album = Some(album.to_string());
        }

        return Ok(mp3_file);
    }
}
