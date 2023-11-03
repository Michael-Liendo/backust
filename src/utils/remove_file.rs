use std::{fs, path::PathBuf};

pub fn remove_file(from_path: String, to_directory: PathBuf) {
    let binding = PathBuf::from(from_path);
    let file_name = binding.file_name().unwrap().to_str().unwrap();
    let to_path = to_directory.join(file_name);

    if to_path.exists() {
        fs::remove_file(to_path).unwrap();
    }
}
