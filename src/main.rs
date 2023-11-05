mod utils;

use std::collections::HashSet;
use std::{fs, path, thread, time};

use utils::copy_file::copy_file;
use utils::get_last_modification::get_last_modification;
use utils::load_files::load_files;
use utils::remove_file::remove_file;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut backup_dir_path = String::new();
    let mut source_dir_path = String::new();
    let sleep_duration = time::Duration::from_millis(1000);

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

    let mut last_modification = get_last_modification(source_directory.clone());

    let mut initial_files: HashSet<String> = load_files(&source_directory);

    // Copy the initial files
    for file_path in initial_files.clone() {
        let file_path = fs::canonicalize(file_path).unwrap();
        copy_file(&source_directory, &file_path, &backup_directory);
    }
    println!("Initial files copied");

    println!("Watching {} for changes", source_directory.display());

    // Main loop
    loop {
        let metadata = fs::metadata(&source_directory).unwrap();
        let last_modified = metadata.modified().unwrap();
        if last_modification < last_modified {
            last_modification = get_last_modification(source_directory.clone());

            let current_files = load_files(&source_directory);

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
                    let file_path = fs::canonicalize(file_path).unwrap();
                    copy_file(&source_directory, &file_path, &backup_directory);
                }
            }

            if !deleted_files.is_empty() {
                for file_path in deleted_files {
                    let file_path = path::PathBuf::from(file_path);
                    remove_file(&source_directory, &file_path, &backup_directory);
                }
            }

            thread::sleep(sleep_duration);
        }
    }
}
