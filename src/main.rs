use std::env;
use std::fs;
use std::io::{self, ErrorKind};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let binding = String::from("file.txt");
    let file_name = args.get(1).unwrap_or(&binding);

    let mut content = read_or_create_file(file_name)?;

    println!("The contents of the file are: {content:?}");

    println!("Enter text to append(or leave empty to skip)");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input)?;

    if !input.trim().is_empty() {
        content.push_str(&input);
        fs::write("file_name", content)?;
        println!("Content saved!");
    }

    Ok(())
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

fn read_with_line_numbers(path: &str) -> io::Result<()> {
    let content = fs::read_to_string(path)?;
    for (i, line) in content.lines().enumerate() {
        println!("{:4}", i+1, line);
    }

    Ok(())
}
