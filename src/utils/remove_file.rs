use std::fs;
use std::path::{Path, PathBuf};

pub fn remove_file(source_base_path: PathBuf, from_path: PathBuf, to_directory: PathBuf) {
    let relative_path = from_path.strip_prefix(&source_base_path).unwrap();
    let destination_path = to_directory.join(relative_path);

    if destination_path.is_file() {
        fs::remove_file(&destination_path).unwrap();
    } else if destination_path.is_dir() {
        fs::remove_dir_all(&destination_path).unwrap();
    }
}
