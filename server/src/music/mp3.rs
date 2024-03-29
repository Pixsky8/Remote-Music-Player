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
    path: Option<String>,
    delete_afterward: bool,
    skip_votes: u32,
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
            path: None,
            delete_afterward: delete_afterward,
            skip_votes: 0,
        };

        if let Some(name) = tag.title() {
            mp3_file.name = name.to_string();
            mp3_file.path = Some(path.to_string());
        }
        else {
            mp3_file.name = path.to_string();
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
            path: None,
            delete_afterward: false,
            skip_votes: 0,
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

    /**
     * @brief Return true if the name is also the path, false otherwise
     */
    pub fn path_as_name(&self) -> bool {
        self.path == None
    }

    pub fn path_get(&self) -> String {
        match &self.path {
            None => self.name.clone(),
            Some(v) => v.to_string(),
        }
    }

    /**
     * @brief Increment number of skip and return the new number
     */
    pub fn increment_skip(&mut self) -> u32 {
        self.skip_votes += 1;
        self.skip_votes
    }
}
