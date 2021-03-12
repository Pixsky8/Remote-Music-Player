use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

pub fn allow_access(file_path_str: &str) -> bool {
    let file_path: &Path = Path::new(file_path_str);

    if file_path.is_absolute() {
        return false;
    }

    // check if '..' is in the file path
    let dir_back: &OsStr = OsStr::new("..");
    let mut path_iter = file_path.iter();
    while let Some(parent) = path_iter.next() {
        if parent == dir_back {
            return false;
        }
    }

    true
}

pub fn replace_ext(file_path_str: &str, ext: &str) -> String {
    let mut file_path: PathBuf = PathBuf::from(file_path_str);
    file_path.set_extension(ext);

    file_path.to_str().unwrap().to_string()
}
