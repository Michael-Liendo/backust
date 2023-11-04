mod utils;

use std::collections::HashSet;
use std::time::SystemTime;
use std::{fs, path};
use utils::copy_file::copy_file;
use utils::load_files::load_files;
use utils::remove_file::remove_file;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut backup_dir_path = String::new();
    let mut source_dir_path = String::new();

    // Parse the arguments
    for (i, arg) in args.iter().enumerate() {
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
    }

    // Check if the arguments are empty
    if backup_dir_path.is_empty() {
        println!("Error: -b requires a directory to backup");
        std::process::exit(1);
    }
    if source_dir_path.is_empty() {
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

    let mut initial_files: HashSet<String> = load_files(source_directory.clone());

    loop {
        let metadata = fs::metadata(&source_directory).unwrap();
        let last_modified = metadata.modified().unwrap();
        if current_time < last_modified {
            current_time = SystemTime::now();

            let current_files = load_files(source_directory.clone());

            let current_files_clone = current_files.clone();

            let deleted_files: HashSet<String> = initial_files
                .difference(&current_files_clone)
                .cloned()
                .collect();

            let new_files: HashSet<String> =
                current_files.difference(&initial_files).cloned().collect();

            initial_files = current_files;

            if !new_files.is_empty() {
                for file_path in new_files {
                    copy_file(
                        source_directory.clone(),
                        fs::canonicalize(file_path).unwrap(),
                        backup_directory.clone(),
                    );
                }
            }

            if !deleted_files.is_empty() {
                for file_path in deleted_files {
                    remove_file(
                        source_directory.clone(),
                        path::PathBuf::from(file_path),
                        backup_directory.clone(),
                    );
                }
            }
        }
    }
}
