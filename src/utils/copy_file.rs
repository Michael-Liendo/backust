use std::fs;
use std::path::PathBuf;

pub fn copy_file(source_base_path: &PathBuf, from: &PathBuf, to: &PathBuf) {
    if from.is_file() {
        let source_path = from.clone();
        let source_path = source_path.strip_prefix(source_base_path).unwrap();
        let source_path = source_path.to_str().unwrap();
        let destination_path = to.join(source_path);

        if !destination_path.exists() {
            fs::create_dir_all(destination_path.parent().unwrap()).unwrap();
        }

        fs::copy(from, destination_path).unwrap();
    } else {
        for entry in fs::read_dir(from).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            copy_file(source_base_path, &path, to);
        }
    }
}
