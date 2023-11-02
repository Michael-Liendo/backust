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
}
