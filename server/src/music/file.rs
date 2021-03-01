use std::ffi::OsStr;
use std::path::Path;

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
