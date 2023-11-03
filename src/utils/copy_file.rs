use std::fs;
use std::path::PathBuf;

pub fn copy_file(from_path: PathBuf, to_directory: PathBuf) {
    let file_name = from_path.file_name().unwrap().to_str().unwrap();
    let to_path = to_directory.join(file_name);

    fs::copy(from_path, to_path).unwrap();
}
