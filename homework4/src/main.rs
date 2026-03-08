use std::io::{self, Write};
use std::process::Command;

enum FileOperation {
    List(String),
    Display(String),
    Create(String, String),
    Remove(String),
    Pwd,
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(directory_path) => {
            let status = Command::new("ls")
                .arg(&directory_path)
                .status()
                .expect("Failed to execute ls");
            if !status.success() {
                eprintln!("Error: ls command failed for '{}'", directory_path);
            }
        }
        FileOperation::Display(file_path) => {
            let status = Command::new("cat")
                .arg(&file_path)
                .status()
                .expect("Failed to execute cat");
            if !status.success() {
                eprintln!("Error: cat command failed for '{}'", file_path);
            }
        }
        FileOperation::Create(file_path, content) => {
            let user_command = format!("echo '{}' > {}", content, file_path);
            let output = Command::new("sh")
                .arg("-c")
                .arg(&user_command)
                .output()
                .expect("Failed to execute command");
            if output.status.success() {
                println!("\nFile '{}' created successfully.", file_path);
            } else {
                eprintln!("Failed to create file '{}'", file_path);
            }
        }
        FileOperation::Remove(file_path) => {
            let status = Command::new("rm")
                .arg(&file_path)
                .status()
                .expect("Failed to execute rm");
            if status.success() {
                println!("\nFile '{}' removed successfully.", file_path);
            } else {
                eprintln!("Error: Failed to remove '{}'", file_path);
            }
        }
        FileOperation::Pwd => {
            let status = Command::new("pwd")
                .status()
                .expect("Failed to execute pwd");
            if !status.success() {
                eprintln!("Error: pwd command failed");
            }
        }
    }
}

fn display_menu() {
    println!("\nFile Operations Menu:");
    println!("1. List files in a directory");
    println!("2. Display file contents");
    println!("3. Create a new file");
    println!("4. Remove a file");
    println!("5. Print working directory");
    println!("0. Exit");
    println!();
}

fn main() {
    println!("Welcome to the File Operations Program!");

    loop {
        display_menu();
        let choice = read_input("Enter your choice (0-5): ");

        match choice.as_str() {
            "0" => {
                println!("\nGoodbye!");
                break;
            }
            "1" => {
                let dir = read_input("Enter directory path: ");
                perform_operation(FileOperation::List(dir));
            }
            "2" => {
                let path = read_input("Enter file path: ");
                perform_operation(FileOperation::Display(path));
            }
            "3" => {
                let path = read_input("Enter file path: ");
                let content = read_input("Enter content: ");
                perform_operation(FileOperation::Create(path, content));
            }
            "4" => {
                let path = read_input("Enter file path: ");
                perform_operation(FileOperation::Remove(path));
            }
            "5" => {
                perform_operation(FileOperation::Pwd);
            }
            _ => {
                println!("Invalid choice. Please enter a number between 0 and 5.");
            }
        }
    }
}