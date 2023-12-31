use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

pub fn get_last_modification(path: PathBuf) -> SystemTime {
    let metadata = fs::metadata(path).unwrap();

    metadata.modified().unwrap()
}
