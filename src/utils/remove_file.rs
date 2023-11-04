use std::{fs, path::PathBuf};

pub fn remove_file(source_base_path: PathBuf, from_path: String, to_directory: PathBuf) {
    let from_path = PathBuf::from(from_path);
    let from_path = from_path.strip_prefix(&source_base_path).unwrap();
    let destination_path = to_directory.join(from_path);

    if from_path.is_file() {
        fs::remove_file(destination_path).unwrap();
    } else {
        for entry in fs::read_dir(from_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            remove_file(
                source_base_path.clone(),
                path.display().to_string(),
                to_directory.clone(),
            );
        }

        fs::remove_dir(destination_path).unwrap();
    }
}
