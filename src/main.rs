mod user_input;
mod hide_unhide;
mod test;

use user_input::*;
use hide_unhide::*;

fn main() {
    loop {
        let operation: &str = &get_operation_from_user();
        
        if operation == "exit" {
            println!("Ending application...");
            break;
        } else{
            let directory = get_directory_from_user();

        match operation {
            "hide" => {
                match hide_directory(&directory) {
                    Ok(_) => println!(),
                    Err(err) => println!("Error hiding directory: {}", err),
                }
            },
            "unhide" => {
                match unhide_directory(&directory) {
                    Ok(_) => println!(),
                    Err(err) => println!("Error unhiding directory: {}", err),
                }
            },
            _ => continue, // Ask user again to input valid operation
        }

        }

        if !ask_user_to_continue() {
            break;
        }
    }
}

fn ask_user_to_continue() -> bool {
    loop {
        let mut input = String::new();
        println!("Do you want to hide/unhide another directory? (y/n)");
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim().to_lowercase();

        match input.as_str() {
            "y" | "yes" => return true,
            "n" | "no" => {
                println!("Ending application...");
                return false},
            _ => {
                println!("Invalid input. Please enter 'y' or 'n'.");
                continue;
            }
        }
    }
}
