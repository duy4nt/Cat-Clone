use serde::{Deserialize, Serialize}
use std::env;
use std::fs;
use std::io::{self, ErrorKind, Write};

#[derive(Serialize, Deserialize)]
struct Config {
    default_file: String,
    auto_backup: bool,
    line_numbers: bool
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    match args[1].as_str() {
        "read" => {
            let file_name = args.get(2).map(|s| s.as_str()).unwrap_or("file.txt");
            let content = read_or_create_file(file_name)?;
            println!("Content of the file is {}", content);
        }
        "write" => {
            let file_name = args.get(2).map(|s| s.as_str()).unwrap_or("file.txt");
            println!("Input the text to be written:");
            let mut input: String = String::new();
            io::stdin().read_line(&mut input)?;
            fs::write(file_name, input)?;
            println!("Content written to {}", file_name);
        }
        "show_file_info" {

        }
        "search_and_replace" {

        }
        "encrypt" {
            let file_name = args.get(2).map(|s| s.as_str()).unwrap_or("file.txt");
            println!("Enter the key");
            let mut key = String::new();
            io::stdin().read_line(&mut key)?;
            let key = key.trim();

            encrypt_file(file_name, key);
            println!("{} was successfully encrypted");
        }
        "decrypt" {

        }
        "read_with_line_number" {

        }
        "file_stat" {

        }

    }


    Ok(())
}

fn print_usage() {
    println!("Usage:");
    println!("  cargo run read [filename]                   - Read a file");
    println!("  cargo run write [filename]                  - Write to a file");
    println!("  cargo run show_file_info [filename]         - Print the Metadata of the file");
    println!("  cargo run search_and_replace [filename]     - Search and Replaces a string from the file");
    println!("  cargo run encrypt [filename]                - Encrypt a file");
    println!("  cargo run decrypt [filename]                - Decrypt a file");
    println!("  cargo run read_with_line_number [filename]  - Read a file with line number");
    println!("  cargo run file_stats [filename]             - Prints the statistics of the file");
}

fn read_or_create_file(path: &str) -> io::Result<String> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(error) if error.kind() == ErrorKind::NotFound => {
            fs::write(path, "")?;
            Ok(String::new())
        }
        Err(error) => Err(error),
    }
}

fn show_file_info(path: &str) -> io::Result {
    let metadata = fs::metadata(path)?;

    println!("File: {}", path);
    println!("Size: {} bytes", metadata.len());
    println!("Read-only: {}", metadata.permissions().readonly());
    println!("Modified: {:?}", metadata.modified()?);

    Ok(())
}

fn search_and_replace(path: &str, search: &str, replace: &str) -> io::Result {
    let content = fs::read_to_string(path)?;
    let new_content = content.replace(search, replace);
    
    fs::write(path, new_content)?;
    println!("Replaced all occurrences of {} with {}", search, replace);
    
    Ok(())
}

fn encrypt_file(file_name: &str, key: &str ) -> io::Result<()> {
    let content = fs::read(file_name)?;
    let encrypted = xor_cipher(&content, key.as_bytes());

    let encrypted_path = format!("{}.enc", file_name);
    fs::write(&encrypted_path, encrypted);

    println!("Delete original file? (y/n):");
    let mut response = String::new();
    io::stdin().read_line(&mut response)?;

    if response.trim().eq_ignore_ascii_case("y") {
        fs::remove_file(file_name)?;
        println!("Original file deleted");
    }
}

fn read_with_line_numbers(path: &str) -> io::Result<()> {
    let content = fs::read_to_string(path)?;
    for (i, line) in content.lines().enumerate() {
        println!("{:4}", i+1, line);
    }

    Ok(())
}

fn file_stats(path: &str) -> io::Result<()> {
    let content = fd::read_to_string(path)?;

    println!("No of Lines: {}", content.lines().count());
    println!("No of Words: {}", content.split_whitespaces().count());
    println!("No of Characters: {}", content.chars().count());

    Ok(())
}
