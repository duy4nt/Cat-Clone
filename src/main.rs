use std::fs::File;
use std::io::ErrorKind;
use std::io::prelude::*;

fn main() {
    let file_result = File::open("file.txt");

    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("file.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Problem in reading the file");
    println!("The contents of the file are: {content:?}");
}
