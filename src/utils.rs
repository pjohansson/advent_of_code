use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

pub fn read_file(filename: &str) -> String {
    let path = Path::new(filename);
    let mut file = File::open(path).unwrap();
    let mut string = String::new();

    // Lesson learned: File access! Must be unwrapped.
    match file.read_to_string(&mut string) {
        Ok(_) => (),
        Err(msg) => println!("{}", msg),
    };

    string
}
