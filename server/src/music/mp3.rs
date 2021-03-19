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
    pub fn from_path(
        path: &str,
        delete_afterward: bool,
    ) -> Result<Mp3, Box<dyn Error>> {
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

    pub fn new(
        name: String,
        author: Option<String>,
        album: Option<String>,
        album_cover: Option<String>,
    ) -> Mp3 {
        Mp3 {
            name: name,
            author: author,
            album: album,
            album_cover: album_cover,
            is_path: false,
            delete_afterward: false,
        }
    }

    pub fn name_get(&self) -> String {
        self.name.clone()
    }

    pub fn author_get(&self) -> Option<String> {
        if self.author.is_none() {
            return None;
        }

        return Some(self.author.as_ref().unwrap().clone());
    }

    pub fn album_get(&self) -> Option<String> {
        if self.album.is_none() {
            return None;
        }

        return Some(self.album.as_ref().unwrap().clone());
    }

    pub fn cover_get(&self) -> Option<String> {
        if self.album_cover.is_none() {
            return None;
        }

        return Some(self.album_cover.as_ref().unwrap().clone());
    }

    pub fn is_temporary(&self) -> bool {
        self.delete_afterward
    }

    pub fn use_path(&self) -> bool {
        self.is_path
    }
}
