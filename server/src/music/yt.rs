use std::process::Command;
use serde_json::Value;

use crate::music::file;

/**
 * @brief Start youtube-dl and return the name of the downloaded file
 */
pub fn yt_dl(url: &str) -> Option<String> {
    let cmd = Command::new("youtube-dl")
        .arg("-f")
        .arg("bestaudio")
        .arg("--print-json")
        .arg("-o")
        .arg("/tmp/ytdl/%(title)s_%(id)s.%(ext)s")
        .arg("-x")
        .arg("--audio-format")
        .arg("mp3")
        .arg(url)
        .output()
        .expect("failed to execute process");

    if !cmd.status.success() {
        println!("youtube-dl failure");
        return None;
    }

    let output: Vec<u8> = cmd.stdout;
    let output_str: &str = match std::str::from_utf8(&output) {
        Ok(v) => v,
        Err(e) => {
            println!("Invalid UTF-8 sequence: {}", e);
            return None;
        },
    };

    let json_out: Value = match serde_json::from_str(output_str) {
        Ok(v) => v,
        Err(e) => {
            println!("Invalid JSON File: {}", e);
            return None;
        },
    };

    let file_name: Option<&str> = json_out["_filename"].as_str();
    if file_name == None {
        return None
    };

    return Some(file::replace_ext(file_name.unwrap(), "mp3"));
}
