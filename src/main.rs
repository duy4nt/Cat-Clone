use std::env;
use std::fs;
use std::io::{self, ErrorKind};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let binding = String::from("file.txt");
    let file_name = args.get(1).unwrap_or(&binding);

    let content = read_or_create_file(file_name)?;

    println!("The contents of the file are: {content:?}");

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
