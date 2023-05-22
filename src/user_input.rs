use std::io::Write;
use std::path::{Path, PathBuf};

pub fn get_operation_from_user() -> String {
    loop {
        let mut operation = String::new();
        println!("Enter 'hide' or 'unhide': ");
        std::io::stdin().read_line(&mut operation).expect("Failed to read line");

        let operation = operation.trim().to_lowercase();

        if operation == "hide" || operation == "unhide" || operation == "exit" {
            return operation;
        } else {
            println!("Invalid operation. Enter 'hide' or 'unhide' to continue or 'exit' to end program");
        }
    }
}


pub fn get_directory_from_user() -> PathBuf {
    loop {
        let mut directory = String::new();
        print!("Enter directory path: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut directory).expect("Failed to read line");

        let directory = directory.trim().trim_matches('"'); // remove leading/trailing whitespace and quotes

        let file_path = Path::new(directory);

        if !file_path.exists() {
            println!("Directory '{}' does not exist. Please try again.", directory);
            continue;
        } else if !file_path.is_dir() {
            println!("'{}' is not a directory. Please try again.", directory);
            continue;
        } else {
            return file_path.to_path_buf();
        }
    }
}