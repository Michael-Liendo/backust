use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut backup_dir_path = String::new();
    let mut source_dir_path = String::new();
    let mut i = 0;

    // Parse the arguments
    for arg in &args {
        if arg == "-b" {
            if i + 1 >= args.len() {
                println!("Error: -b requires a directory to backup");
                std::process::exit(1);
            }
            backup_dir_path = args[i + 1].clone();
        }
        if arg == "-s" {
            if i + 1 >= args.len() {
                println!("Error: -s requires a directory to source");
                std::process::exit(1);
            }
            source_dir_path = args[i + 1].clone();
        }
        if arg == "-h" {
            println!("Usage:  -b <backup_dir> -s <source_dir>");
            std::process::exit(0);
        }
        i += 1;
    }

    // Check if the arguments are empty
    if backup_dir_path == "" {
        println!("Error: -b requires a directory to backup");
        std::process::exit(1);
    }
    if source_dir_path == "" {
        println!("Error: -s requires a directory to source");
        std::process::exit(1);
    }

    // Convert to absolute paths
    let backup_directory = fs::canonicalize(backup_dir_path).unwrap();
    let source_directory = fs::canonicalize(source_dir_path).unwrap();

    // Check if the paths are directories
    if !backup_directory.is_dir() {
        println!(
            "Error: the backup path is is not a directory, {}",
            backup_directory.display()
        );
        std::process::exit(1);
    }
    if !source_directory.is_dir() {
        println!(
            "Error: the source path is not a directory, {}",
            source_directory.display()
        );
        std::process::exit(1);
    }

    // Check if the paths are the same
    if source_directory == backup_directory {
        println!("Error: the source and backup paths are the same");
        std::process::exit(1);
    }

    println!("Watching {} for changes", source_directory.display());

    let mut current_time = SystemTime::now();

    loop {
        let metadata = fs::metadata(&source_directory).unwrap();
        let last_modified = metadata.modified().unwrap();
        if current_time < last_modified {
            current_time = SystemTime::now();

            println!(
                "Copying {} to {}",
                source_directory.display(),
                backup_directory.display()
            );

            copy(source_directory.clone(), backup_directory.clone());
        }
    }
}

fn copy(from_directory: PathBuf, to_directory: PathBuf) {
    for entry in fs::read_dir(&from_directory).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            copy(path.clone(), to_directory.join(path.file_name().unwrap()));
        } else {
            fs::copy(path.clone(), to_directory.join(path.file_name().unwrap())).unwrap();
        }
    }
}
