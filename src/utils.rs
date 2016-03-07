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


#[cfg(test)]
pub mod tests {
    // Lesson learned: Structs and string slices must share their lifetime
    // Lesson learned: To use struct in another module, fields must be public
    pub struct Expected<'a> {
        pub input: &'a str,
        pub result: i32
    }

    pub fn assert_one_expected(check: &Expected, func: fn(&String) -> i32) {
        match check {
            // Lesson learned: Pattern matching against Structs
            // go in variable order
            &Expected { input, result } => {
                assert_eq!(func(&input.to_string()), result);
            },
        };
    }

    // Lesson learned: Functions accept arrays as references and functions by type signature
    pub fn assert_all_expected(expected_answers: &[Expected], func: fn(&String) -> i32) {
        // Lesson learned: Loop iterators return a reference
        // This must be pattern matched by getting the object on the other side
        for check in expected_answers {
            assert_one_expected(check, func);
        }
    }
}

