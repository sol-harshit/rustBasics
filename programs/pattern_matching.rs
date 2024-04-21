// use pattern matching to handle different errors returned from a mock file read operation.

use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn read_file_contents(path: &str) -> Result<String, io::Error> {
    let mut file = match File::open(path) {
            Ok(file) => file,
            Err(err) => return Err(err),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(err) => Err(err),
    }
}

fn main() {
    match read_file_contents("non_existent_file.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => println!("File not found"),
            other_error => println!("Error reading file: {:?}", other_error),
        },
    }    
}
