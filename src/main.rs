use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut backup_dir = String::new();
    let mut source_dir = String::new();
    let mut i = 0;

    // Parse the arguments
    for arg in &args {
        if arg == "-b" {
            if i + 1 >= args.len() {
                println!("Error: -b requires a directory to backup");
                std::process::exit(1);
            }
            backup_dir = args[i + 1].clone();
        }
        if arg == "-s" {
            if i + 1 >= args.len() {
                println!("Error: -s requires a directory to source");
                std::process::exit(1);
            }
            source_dir = args[i + 1].clone();
        }
        if arg == "-h" {
            println!("Usage:  -b <backup_dir> -s <source_dir>");
            std::process::exit(0);
        }
        i += 1;
    }

    // Check if the arguments are empty
    if backup_dir == "" {
        println!("Error: -b requires a directory to backup");
        std::process::exit(1);
    }
    if source_dir == "" {
        println!("Error: -s requires a directory to source");
        std::process::exit(1);
    }

    // Convert to absolute paths
    let backup_dir = fs::canonicalize(backup_dir).unwrap();
    let source_dir = fs::canonicalize(source_dir).unwrap();

    // Check if the paths are directories
    if !backup_dir.is_dir() {
        println!(
            "Error: the backup path is is not a directory, {}",
            backup_dir.display()
        );
        std::process::exit(1);
    }
    if !source_dir.is_dir() {
        println!(
            "Error: the source path is not a directory, {}",
            source_dir.display()
        );
        std::process::exit(1);
    }

    // Check if the paths are the same
    if source_dir == backup_dir {
        println!("Error: the source and backup paths are the same");
        std::process::exit(1);
    }

    //
}
