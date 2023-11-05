use std::{collections::HashSet, fs, path::PathBuf};

pub fn load_files(directory: &PathBuf) -> HashSet<String> {
    let mut files: HashSet<String> = HashSet::new();

    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        files.insert(path.display().to_string());

        if path.is_dir() {
            files.extend(load_files(&path));
        }
    }

    files
}
